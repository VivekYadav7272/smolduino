#![allow(clippy::upper_case_acronyms)]

use super::reg_io::{private::Sealed, RegisterMapping};
use avrd::current as inner;
pub struct LOW;
impl Sealed for LOW {}
impl RegisterMapping for LOW {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::LOW
    }
}

pub struct LOCKBIT;
impl Sealed for LOCKBIT {}
impl RegisterMapping for LOCKBIT {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::LOCKBIT
    }
}

pub struct HIGH;
impl Sealed for HIGH {}
impl RegisterMapping for HIGH {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::HIGH
    }
}

pub struct EXTENDED;
impl Sealed for EXTENDED {}
impl RegisterMapping for EXTENDED {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::EXTENDED
    }
}

pub struct PINB;
impl Sealed for PINB {}
impl RegisterMapping for PINB {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PINB
    }
}

pub struct DDRB;
impl Sealed for DDRB {}
impl RegisterMapping for DDRB {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::DDRB
    }
}

pub struct PORTB;
impl Sealed for PORTB {}
impl RegisterMapping for PORTB {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PORTB
    }
}

pub struct PINC;
impl Sealed for PINC {}
impl RegisterMapping for PINC {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PINC
    }
}

pub struct DDRC;
impl Sealed for DDRC {}
impl RegisterMapping for DDRC {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::DDRC
    }
}

pub struct PORTC;
impl Sealed for PORTC {}
impl RegisterMapping for PORTC {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PORTC
    }
}

pub struct PIND;
impl Sealed for PIND {}
impl RegisterMapping for PIND {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PIND
    }
}

pub struct DDRD;
impl Sealed for DDRD {}
impl RegisterMapping for DDRD {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::DDRD
    }
}

pub struct PORTD;
impl Sealed for PORTD {}
impl RegisterMapping for PORTD {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PORTD
    }
}

pub struct TIFR0;
impl Sealed for TIFR0 {}
impl RegisterMapping for TIFR0 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TIFR0
    }
}

pub struct TIFR1;
impl Sealed for TIFR1 {}
impl RegisterMapping for TIFR1 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TIFR1
    }
}

pub struct TIFR2;
impl Sealed for TIFR2 {}
impl RegisterMapping for TIFR2 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TIFR2
    }
}

pub struct PCIFR;
impl Sealed for PCIFR {}
impl RegisterMapping for PCIFR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PCIFR
    }
}

pub struct EIFR;
impl Sealed for EIFR {}
impl RegisterMapping for EIFR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::EIFR
    }
}

pub struct EIMSK;
impl Sealed for EIMSK {}
impl RegisterMapping for EIMSK {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::EIMSK
    }
}

pub struct GPIOR0;
impl Sealed for GPIOR0 {}
impl RegisterMapping for GPIOR0 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::GPIOR0
    }
}

pub struct EECR;
impl Sealed for EECR {}
impl RegisterMapping for EECR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::EECR
    }
}

pub struct EEDR;
impl Sealed for EEDR {}
impl RegisterMapping for EEDR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::EEDR
    }
}

pub struct EEAR;
impl Sealed for EEAR {}
impl RegisterMapping for EEAR {
    type RegisterType = u16;
    fn get_reg_addr() -> *mut u16 {
        // OMG I was so worried that the pointer to u16 would be 2-byte aligned,
        // hence violating the Rust constraint for references to be aligned to
        // align_of_val(&T). However, to my surprise,
        // align_of::<*mut u16>() == 1. So, I'm good.
        // (If not, then this assertion would fail at compile-time and I would know.)
        // Ik, weird place to put this assertion in, but this is the only
        // Register with a non-2-byte aligned address.
        const _: () = assert!(align_of::<*mut u16>() == 1);
        inner::EEAR // inner::EEAR: *mut u16 = 0x41
    }
}

pub struct EEARL;
impl Sealed for EEARL {}
impl RegisterMapping for EEARL {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::EEARL
    }
}

pub struct EEARH;
impl Sealed for EEARH {}
impl RegisterMapping for EEARH {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::EEARH
    }
}

pub struct GTCCR;
impl Sealed for GTCCR {}
impl RegisterMapping for GTCCR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::GTCCR
    }
}

pub struct TCCR0A;
impl Sealed for TCCR0A {}
impl RegisterMapping for TCCR0A {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR0A
    }
}

pub struct TCCR0B;
impl Sealed for TCCR0B {}
impl RegisterMapping for TCCR0B {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR0B
    }
}

pub struct TCNT0;
impl Sealed for TCNT0 {}
impl RegisterMapping for TCNT0 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCNT0
    }
}

pub struct OCR0A;
impl Sealed for OCR0A {}
impl RegisterMapping for OCR0A {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::OCR0A
    }
}

pub struct OCR0B;
impl Sealed for OCR0B {}
impl RegisterMapping for OCR0B {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::OCR0B
    }
}

pub struct GPIOR1;
impl Sealed for GPIOR1 {}
impl RegisterMapping for GPIOR1 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::GPIOR1
    }
}

pub struct GPIOR2;
impl Sealed for GPIOR2 {}
impl RegisterMapping for GPIOR2 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::GPIOR2
    }
}

pub struct SPCR;
impl Sealed for SPCR {}
impl RegisterMapping for SPCR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::SPCR
    }
}

pub struct SPSR;
impl Sealed for SPSR {}
impl RegisterMapping for SPSR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::SPSR
    }
}

pub struct SPDR;
impl Sealed for SPDR {}
impl RegisterMapping for SPDR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::SPDR
    }
}

pub struct ACSR;
impl Sealed for ACSR {}
impl RegisterMapping for ACSR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::ACSR
    }
}

pub struct SMCR;
impl Sealed for SMCR {}
impl RegisterMapping for SMCR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::SMCR
    }
}

pub struct MCUSR;
impl Sealed for MCUSR {}
impl RegisterMapping for MCUSR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::MCUSR
    }
}

pub struct MCUCR;
impl Sealed for MCUCR {}
impl RegisterMapping for MCUCR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::MCUCR
    }
}

pub struct SPMCSR;
impl Sealed for SPMCSR {}
impl RegisterMapping for SPMCSR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::SPMCSR
    }
}

pub struct SPL;
impl Sealed for SPL {}
impl RegisterMapping for SPL {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::SPL
    }
}

pub struct SP;
impl Sealed for SP {}
impl RegisterMapping for SP {
    type RegisterType = u16;
    fn get_reg_addr() -> *mut u16 {
        inner::SP
    }
}

pub struct SPH;
impl Sealed for SPH {}
impl RegisterMapping for SPH {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::SPH
    }
}

pub struct SREG;
impl Sealed for SREG {}
impl RegisterMapping for SREG {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::SREG
    }
}

pub struct WDTCSR;
impl Sealed for WDTCSR {}
impl RegisterMapping for WDTCSR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::WDTCSR
    }
}

pub struct CLKPR;
impl Sealed for CLKPR {}
impl RegisterMapping for CLKPR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::CLKPR
    }
}

pub struct PRR;
impl Sealed for PRR {}
impl RegisterMapping for PRR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PRR
    }
}

pub struct OSCCAL;
impl Sealed for OSCCAL {}
impl RegisterMapping for OSCCAL {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::OSCCAL
    }
}

pub struct PCICR;
impl Sealed for PCICR {}
impl RegisterMapping for PCICR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PCICR
    }
}

pub struct EICRA;
impl Sealed for EICRA {}
impl RegisterMapping for EICRA {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::EICRA
    }
}

pub struct PCMSK0;
impl Sealed for PCMSK0 {}
impl RegisterMapping for PCMSK0 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PCMSK0
    }
}

pub struct PCMSK1;
impl Sealed for PCMSK1 {}
impl RegisterMapping for PCMSK1 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PCMSK1
    }
}

pub struct PCMSK2;
impl Sealed for PCMSK2 {}
impl RegisterMapping for PCMSK2 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::PCMSK2
    }
}

pub struct TIMSK0;
impl Sealed for TIMSK0 {}
impl RegisterMapping for TIMSK0 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TIMSK0
    }
}

pub struct TIMSK1;
impl Sealed for TIMSK1 {}
impl RegisterMapping for TIMSK1 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TIMSK1
    }
}

pub struct TIMSK2;
impl Sealed for TIMSK2 {}
impl RegisterMapping for TIMSK2 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TIMSK2
    }
}

pub struct ADCL;
impl Sealed for ADCL {}
impl RegisterMapping for ADCL {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::ADCL
    }
}

pub struct ADC;
impl Sealed for ADC {}
impl RegisterMapping for ADC {
    type RegisterType = u16;
    fn get_reg_addr() -> *mut u16 {
        inner::ADC
    }
}

pub struct ADCH;
impl Sealed for ADCH {}
impl RegisterMapping for ADCH {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::ADCH
    }
}

pub struct ADCSRA;
impl Sealed for ADCSRA {}
impl RegisterMapping for ADCSRA {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::ADCSRA
    }
}

pub struct ADCSRB;
impl Sealed for ADCSRB {}
impl RegisterMapping for ADCSRB {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::ADCSRB
    }
}

pub struct ADMUX;
impl Sealed for ADMUX {}
impl RegisterMapping for ADMUX {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::ADMUX
    }
}

pub struct DIDR0;
impl Sealed for DIDR0 {}
impl RegisterMapping for DIDR0 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::DIDR0
    }
}

pub struct DIDR1;
impl Sealed for DIDR1 {}
impl RegisterMapping for DIDR1 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::DIDR1
    }
}

pub struct TCCR1A;
impl Sealed for TCCR1A {}
impl RegisterMapping for TCCR1A {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR1A
    }
}

pub struct TCCR1B;
impl Sealed for TCCR1B {}
impl RegisterMapping for TCCR1B {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR1B
    }
}

pub struct TCCR1C;
impl Sealed for TCCR1C {}
impl RegisterMapping for TCCR1C {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR1C
    }
}

pub struct TCNT1L;
impl Sealed for TCNT1L {}
impl RegisterMapping for TCNT1L {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCNT1L
    }
}

pub struct TCNT1;
impl Sealed for TCNT1 {}
impl RegisterMapping for TCNT1 {
    type RegisterType = u16;
    fn get_reg_addr() -> *mut u16 {
        inner::TCNT1
    }
}

pub struct TCNT1H;
impl Sealed for TCNT1H {}
impl RegisterMapping for TCNT1H {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCNT1H
    }
}

pub struct ICR1L;
impl Sealed for ICR1L {}
impl RegisterMapping for ICR1L {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::ICR1L
    }
}

pub struct ICR1;
impl Sealed for ICR1 {}
impl RegisterMapping for ICR1 {
    type RegisterType = u16;
    fn get_reg_addr() -> *mut u16 {
        inner::ICR1
    }
}

pub struct ICR1H;
impl Sealed for ICR1H {}
impl RegisterMapping for ICR1H {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::ICR1H
    }
}

pub struct OCR1AL;
impl Sealed for OCR1AL {}
impl RegisterMapping for OCR1AL {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::OCR1AL
    }
}

pub struct OCR1A;
impl Sealed for OCR1A {}
impl RegisterMapping for OCR1A {
    type RegisterType = u16;
    fn get_reg_addr() -> *mut u16 {
        inner::OCR1A
    }
}

pub struct OCR1AH;
impl Sealed for OCR1AH {}
impl RegisterMapping for OCR1AH {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::OCR1AH
    }
}

pub struct OCR1BL;
impl Sealed for OCR1BL {}
impl RegisterMapping for OCR1BL {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::OCR1BL
    }
}

pub struct OCR1B;
impl Sealed for OCR1B {}
impl RegisterMapping for OCR1B {
    type RegisterType = u16;
    fn get_reg_addr() -> *mut u16 {
        inner::OCR1B
    }
}

pub struct OCR1BH;
impl Sealed for OCR1BH {}
impl RegisterMapping for OCR1BH {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::OCR1BH
    }
}

pub struct TCCR2A;
impl Sealed for TCCR2A {}
impl RegisterMapping for TCCR2A {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR2A
    }
}

pub struct TCCR2B;
impl Sealed for TCCR2B {}
impl RegisterMapping for TCCR2B {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR2B
    }
}

pub struct TCNT2;
impl Sealed for TCNT2 {}
impl RegisterMapping for TCNT2 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TCNT2
    }
}

pub struct OCR2A;
impl Sealed for OCR2A {}
impl RegisterMapping for OCR2A {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::OCR2A
    }
}

pub struct OCR2B;
impl Sealed for OCR2B {}
impl RegisterMapping for OCR2B {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::OCR2B
    }
}

pub struct ASSR;
impl Sealed for ASSR {}
impl RegisterMapping for ASSR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::ASSR
    }
}

pub struct TWBR;
impl Sealed for TWBR {}
impl RegisterMapping for TWBR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TWBR
    }
}

pub struct TWSR;
impl Sealed for TWSR {}
impl RegisterMapping for TWSR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TWSR
    }
}

pub struct TWAR;
impl Sealed for TWAR {}
impl RegisterMapping for TWAR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TWAR
    }
}

pub struct TWDR;
impl Sealed for TWDR {}
impl RegisterMapping for TWDR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TWDR
    }
}

pub struct TWCR;
impl Sealed for TWCR {}
impl RegisterMapping for TWCR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TWCR
    }
}

pub struct TWAMR;
impl Sealed for TWAMR {}
impl RegisterMapping for TWAMR {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::TWAMR
    }
}

pub struct UCSR0A;
impl Sealed for UCSR0A {}
impl RegisterMapping for UCSR0A {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::UCSR0A
    }
}

pub struct UCSR0B;
impl Sealed for UCSR0B {}
impl RegisterMapping for UCSR0B {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::UCSR0B
    }
}

pub struct UCSR0C;
impl Sealed for UCSR0C {}
impl RegisterMapping for UCSR0C {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::UCSR0C
    }
}

pub struct UBRR0;
impl Sealed for UBRR0 {}
impl RegisterMapping for UBRR0 {
    type RegisterType = u16;
    fn get_reg_addr() -> *mut u16 {
        inner::UBRR0
    }
}

pub struct UBRR0L;
impl Sealed for UBRR0L {}
impl RegisterMapping for UBRR0L {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::UBRR0L
    }
}

pub struct UBRR0H;
impl Sealed for UBRR0H {}
impl RegisterMapping for UBRR0H {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::UBRR0H
    }
}

pub struct UDR0;
impl Sealed for UDR0 {}
impl RegisterMapping for UDR0 {
    type RegisterType = u8;
    fn get_reg_addr() -> *mut u8 {
        inner::UDR0
    }
}
