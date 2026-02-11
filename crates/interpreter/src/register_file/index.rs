use crate::Exception;

pub trait IndexByRegister<Register, T> {
    fn read(&self, register: Register) -> T;
    fn write(&mut self, register: Register, value: T);
}

pub trait TryIndexByRegister<Register, T> {
    fn try_read(&self, register: Register) -> Result<T, Exception>;
    fn try_write(&mut self, register: Register, value: T) -> Result<(), Exception>;
}
