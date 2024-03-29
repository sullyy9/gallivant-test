use std::io::{self, Read, Write};

use crate::{error::Error, syntax::ParsedExpr};

use super::measurement::{self, Measurement, MeasurementTest};

////////////////////////////////////////////////////////////////
// types
////////////////////////////////////////////////////////////////

/// Describes a two way transaction between a frontend and a device.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    expression: ParsedExpr,
    txbytes: Vec<u8>,
    txcomplete: bool,
    device: Device,
    response: Vec<u8>,
    test: Option<MeasurementTest>,
}

////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub enum TransactionStatus {
    Success,
    Ongoing(Transaction),
}

////////////////////////////////////////////////////////////////

/// Device that a frontend may need to communcate with during script execution.
///
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Device {
    TCU,
    Printer,
}

////////////////////////////////////////////////////////////////
// construction / conversion
////////////////////////////////////////////////////////////////

impl Transaction {
    pub fn with_tcu(
        expression: ParsedExpr,
        txbytes: Vec<u8>,
        test: Option<MeasurementTest>,
    ) -> Self {
        Self {
            expression,
            txbytes,
            txcomplete: false,
            device: Device::TCU,
            response: Vec::new(),
            test,
        }
    }

    pub fn with_printer(
        expression: ParsedExpr,
        txbytes: Vec<u8>,
        test: Option<MeasurementTest>,
    ) -> Self {
        Self {
            expression,
            txbytes,
            txcomplete: false,
            device: Device::Printer,
            response: Vec::new(),
            test,
        }
    }
}

////////////////////////////////////////////////////////////////
// methods
////////////////////////////////////////////////////////////////

impl Transaction {
    pub fn bytes(&self) -> &[u8] {
        &self.txbytes
    }

    pub fn process<T: Read + Write>(mut self, port: &mut T) -> Result<TransactionStatus, Error> {
        let into_io_error = |error| Error::from_io_error(self.expression.clone(), error);

        // Send bytes if needed.
        if !self.txcomplete {
            port.write_all(&self.txbytes).map_err(into_io_error)?;
            self.txcomplete = true;

            return if self.device == Device::Printer && self.test.is_none() {
                Ok(TransactionStatus::Success)
            } else {
                Ok(TransactionStatus::Ongoing(self))
            };
        }

        let response = {
            let mut buffer = [0; 256];
            let count = port.read(&mut buffer).map_err(into_io_error)?;
            buffer[0..count].to_owned()
        };

        self.response.extend_from_slice(&response);
        self.evaluate_response()
    }

    fn evaluate_response(mut self) -> Result<TransactionStatus, Error> {
        // Find the number of expected \r characters.
        let echo_expected = self.device == Device::TCU;
        let expected_endings = if self.test.is_some() && echo_expected {
            2
        } else if self.test.is_some() || echo_expected {
            1
        } else {
            0
        };

        // No response expected.
        if expected_endings == 0 {
            return Ok(TransactionStatus::Success);
        }

        let parts: Vec<&[u8]> = self.response.split_inclusive(|&b| b == b'\r').collect();

        // Incomplete response.
        if parts.len() < expected_endings {
            return Ok(TransactionStatus::Ongoing(self));
        }

        let (echo, measurement) = if echo_expected {
            (parts.get(0), parts.get(1))
        } else {
            (None, parts.get(0))
        };

        // Validate the echo.
        if echo.is_some_and(|echo| *echo != self.txbytes) {
            todo!("Command echo incorrect");
        }

        // Test the measurement.
        if let Some(test) = self.test {
            let measurement = *measurement.unwrap(); // Already checked that the measurement exists.
            let measurement = Measurement::try_from(measurement)
                .unwrap_or_else(|_| todo!("Handle measurement parsing failure"));

            match test.test(measurement) {
                Ok(_) => (),
                Err(measurement::Error::TestFailedRetryable(test)) => {
                    self.test = Some(test);
                    self.txcomplete = false;
                    return Ok(TransactionStatus::Ongoing(self));
                }
                Err(measurement::Error::TestFailed(test)) => {
                    return Err(Error::from_failed_test(self.expression, test))
                }
                _ => todo!(),
            }
        }

        // Success.
        Ok(TransactionStatus::Success)
    }
}

////////////////////////////////////////////////////////////////
