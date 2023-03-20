use core::time::Duration;
use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::RmtChannel;
use esp_idf_hal::rmt::*;
use esp_idf_sys::EspError;
use smart_leds_trait::RGB8;

pub struct LedRmtDriver<'d> {
    tx: TxRmtDriver<'d>,
}

impl<'d> LedRmtDriver<'d> {
    #[inline]
    pub fn new(
        pin: impl Peripheral<P = impl OutputPin> + 'd,
        channel: impl Peripheral<P = impl RmtChannel> + 'd,
    ) -> Result<Self, EspError> {
        esp_idf_hal::into_ref!(pin);

        let config = TransmitConfig::new().clock_divider(1);
        let tx = TxRmtDriver::new(channel, pin, &config)?;

        Ok(Self { tx })
    }

    fn get_data_pulses(&mut self) -> Result<(Pulse, Pulse, Pulse, Pulse), EspError> {
        let ticks_hz = self.tx.counter_clock()?;
        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(350))?;
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(700))?;
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(600))?;
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(500))?;
        return Ok((t0h, t0l, t1h, t1l));
    }

    fn get_rgb_pulses<'a>(
        &'a mut self,
        rgb: RGB8,
        pulses: &'a (Pulse, Pulse, Pulse, Pulse),
    ) -> Vec<&Pulse> {
        let color: u32 = ((rgb.g as u32) << 16) | ((rgb.r as u32) << 8) | rgb.b as u32;
        let (t0h, t0l, t1h, t1l) = pulses;
        let mut pulses = vec![];
        for i in (0..24).rev() {
            let p = 2_u32.pow(i);
            let bit = p & color != 0;
            if bit {
                pulses.push(t1h);
                pulses.push(t1l);
            } else {
                pulses.push(t0h);
                pulses.push(t0l);
            }
        }
        return pulses;
    }

    pub fn write(&mut self, rgb: RGB8) -> Result<(), EspError> {
        let mut signal = VariableLengthSignal::new();
        let pulses = self.get_data_pulses()?;
        let rgb_pulses = self.get_rgb_pulses(rgb, &pulses);
        signal.push(rgb_pulses)?;
        self.tx.start_blocking(&signal)
    }

    pub fn write_n(&mut self, rgb: RGB8, n: u32) -> Result<(), EspError> {
        let mut signal = VariableLengthSignal::new();
        let pulses = self.get_data_pulses()?;
        for _i in 0..n {
            let rgb_pulses = self.get_rgb_pulses(rgb, &pulses);
            signal.push(rgb_pulses)?;
        }
        self.tx.start_blocking(&signal)
    }

    pub fn write_iter<T>(&mut self, rgbs: T) -> Result<(), EspError>
    where
        T: Iterator<Item = RGB8>,
    {
        let mut signal = VariableLengthSignal::new();
        let pulses = self.get_data_pulses()?;
        for rgb in rgbs {
            let rgb_pulses = self.get_rgb_pulses(rgb, &pulses);
            signal.push(rgb_pulses)?;
        }
        self.tx.start_blocking(&signal)
    }
}
