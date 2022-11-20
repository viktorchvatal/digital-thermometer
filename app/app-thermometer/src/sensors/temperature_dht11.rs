use dht11::{Dht11, Measurement};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::blocking::{delay::{DelayUs, DelayMs}};

pub struct Dht11Drivers<T0, T1, T2, T3, T4, T5> {
    sensor0: Dht11<T0>,
    sensor1: Dht11<T1>,
    sensor2: Dht11<T2>,
    sensor3: Dht11<T3>,
    sensor4: Dht11<T4>,
    sensor5: Dht11<T5>,
}

impl<T0, T1, T2, T3, T4, T5, TE> Dht11Drivers<T0, T1, T2, T3, T4, T5>
where
T0: InputPin<Error = TE> + OutputPin<Error = TE>,
T1: InputPin<Error = TE> + OutputPin<Error = TE>,
T2: InputPin<Error = TE> + OutputPin<Error = TE>,
T3: InputPin<Error = TE> + OutputPin<Error = TE>,
T4: InputPin<Error = TE> + OutputPin<Error = TE>,
T5: InputPin<Error = TE> + OutputPin<Error = TE>, {
    pub fn new(
        sensor0: Dht11<T0>,
        sensor1: Dht11<T1>,
        sensor2: Dht11<T2>,
        sensor3: Dht11<T3>,
        sensor4: Dht11<T4>,
        sensor5: Dht11<T5>,
    ) -> Self {
        Self {
            sensor0,
            sensor1,
            sensor2,
            sensor3,
            sensor4,
            sensor5,
        }
    }
}

pub trait Dht11Reader<D>
where D: DelayUs<u16> + DelayMs<u16>{
    fn read(&mut self, delay: &mut D) ->  [Option<Measurement>; 6];
}

impl<T0, T1, T2, T3, T4, T5, TE, D> Dht11Reader<D> for Dht11Drivers<T0, T1, T2, T3, T4, T5>
where
T0: InputPin<Error = TE> + OutputPin<Error = TE>,
T1: InputPin<Error = TE> + OutputPin<Error = TE>,
T2: InputPin<Error = TE> + OutputPin<Error = TE>,
T3: InputPin<Error = TE> + OutputPin<Error = TE>,
T4: InputPin<Error = TE> + OutputPin<Error = TE>,
T5: InputPin<Error = TE> + OutputPin<Error = TE>,
D: DelayUs<u16> + DelayMs<u16> {
    fn read(&mut self, delay: &mut D) -> [Option<Measurement>; 6]
{
        [
            read_dht11(&mut self.sensor0, delay),
            read_dht11(&mut self.sensor1, delay),
            read_dht11(&mut self.sensor2, delay),
            read_dht11(&mut self.sensor3, delay),
            read_dht11(&mut self.sensor4, delay),
            read_dht11(&mut self.sensor5, delay),
        ]
    }
}

fn read_dht11<T, D, E>(
    driver: &mut Dht11<T>,
    delay: &mut D
) -> Option<Measurement>
where
    D: DelayUs<u16> + DelayMs<u16>,
    T: InputPin<Error = E> + OutputPin<Error = E>
{
    driver.perform_measurement(delay).ok()
}