use crate::color::WritableColor;
use core::time::Duration;
use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::RmtChannel;
use esp_idf_hal::rmt::*;
use esp_idf_sys::EspError;

pub struct LedRmtDriver<'d> {
    tx: TxRmtDriver<'d>,
}

impl<'d> LedRmtDriver<'d> {
    #[allow(dead_code)]
    pub fn new(
        pin: impl Peripheral<P = impl OutputPin> + 'd,
        channel: impl Peripheral<P = impl RmtChannel> + 'd,
    ) -> Result<Self, EspError> {
        esp_idf_hal::into_ref!(pin);

        let config = TransmitConfig::new().clock_divider(1);
        let tx = TxRmtDriver::new(channel, pin, &config)?;

        Ok(Self { tx })
    }

    #[allow(dead_code)]
    fn get_data_pulses(&mut self) -> Result<(Pulse, Pulse, Pulse, Pulse), EspError> {
        let ticks_hz = self.tx.counter_clock()?;
        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(350))?;
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(700))?;
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(600))?;
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(500))?;
        Ok((t0h, t0l, t1h, t1l))
    }

    #[allow(dead_code)]
    pub fn write<T: WritableColor>(&mut self, color: &T) -> Result<(), EspError> {
        let mut signal = VariableLengthSignal::new();
        let pulses = self.get_data_pulses()?;
        let color_pulses = color.get_pulses(&pulses);
        signal.push(color_pulses)?;
        self.tx.start_blocking(&signal)
    }
    #[allow(dead_code)]
    pub fn write_n<T: WritableColor>(&mut self, color: &T, n: u32) -> Result<(), EspError> {
        let mut signal = VariableLengthSignal::new();
        let pulses = self.get_data_pulses()?;
        for _i in 0..n {
            let color_pulses = color.get_pulses(&pulses);
            signal.push(color_pulses)?;
        }
        self.tx.start_blocking(&signal)
    }
    #[allow(dead_code)]
    pub fn write_iter<'c, T, C>(&mut self, colors: T) -> Result<(), EspError>
    where
        T: Iterator<Item = &'c C>,
        C: WritableColor + 'c,
    {
        let mut signal = VariableLengthSignal::new();
        let pulses = self.get_data_pulses()?;
        for color in colors {
            let color_pulses = color.get_pulses(&pulses);
            signal.push(color_pulses)?;
        }
        self.tx.start_blocking(&signal)
    }
}
