use super::reg_io::RegisterMapping;
use avrd::current as inner;
pub struct LOW;
impl RegisterMapping for LOW {
    fn get_reg_addr() -> *mut u8 {
        inner::LOW
    }
}

pub struct LOCKBIT;
impl RegisterMapping for LOCKBIT {
    fn get_reg_addr() -> *mut u8 {
        inner::LOCKBIT
    }
}

pub struct HIGH;
impl RegisterMapping for HIGH {
    fn get_reg_addr() -> *mut u8 {
        inner::HIGH
    }
}

pub struct EXTENDED;
impl RegisterMapping for EXTENDED {
    fn get_reg_addr() -> *mut u8 {
        inner::EXTENDED
    }
}

pub struct PINB;
impl RegisterMapping for PINB {
    fn get_reg_addr() -> *mut u8 {
        inner::PINB
    }
}

pub struct DDRB;
impl RegisterMapping for DDRB {
    fn get_reg_addr() -> *mut u8 {
        inner::DDRB
    }
}

pub struct PORTB;
impl RegisterMapping for PORTB {
    fn get_reg_addr() -> *mut u8 {
        inner::PORTB
    }
}

pub struct PINC;
impl RegisterMapping for PINC {
    fn get_reg_addr() -> *mut u8 {
        inner::PINC
    }
}

pub struct DDRC;
impl RegisterMapping for DDRC {
    fn get_reg_addr() -> *mut u8 {
        inner::DDRC
    }
}

pub struct PORTC;
impl RegisterMapping for PORTC {
    fn get_reg_addr() -> *mut u8 {
        inner::PORTC
    }
}

pub struct PIND;
impl RegisterMapping for PIND {
    fn get_reg_addr() -> *mut u8 {
        inner::PIND
    }
}

pub struct DDRD;
impl RegisterMapping for DDRD {
    fn get_reg_addr() -> *mut u8 {
        inner::DDRD
    }
}

pub struct PORTD;
impl RegisterMapping for PORTD {
    fn get_reg_addr() -> *mut u8 {
        inner::PORTD
    }
}

pub struct TIFR0;
impl RegisterMapping for TIFR0 {
    fn get_reg_addr() -> *mut u8 {
        inner::TIFR0
    }
}

pub struct TIFR1;
impl RegisterMapping for TIFR1 {
    fn get_reg_addr() -> *mut u8 {
        inner::TIFR1
    }
}

pub struct TIFR2;
impl RegisterMapping for TIFR2 {
    fn get_reg_addr() -> *mut u8 {
        inner::TIFR2
    }
}

pub struct PCIFR;
impl RegisterMapping for PCIFR {
    fn get_reg_addr() -> *mut u8 {
        inner::PCIFR
    }
}

pub struct EIFR;
impl RegisterMapping for EIFR {
    fn get_reg_addr() -> *mut u8 {
        inner::EIFR
    }
}

pub struct EIMSK;
impl RegisterMapping for EIMSK {
    fn get_reg_addr() -> *mut u8 {
        inner::EIMSK
    }
}

pub struct GPIOR0;
impl RegisterMapping for GPIOR0 {
    fn get_reg_addr() -> *mut u8 {
        inner::GPIOR0
    }
}

pub struct EECR;
impl RegisterMapping for EECR {
    fn get_reg_addr() -> *mut u8 {
        inner::EECR
    }
}

pub struct EEDR;
impl RegisterMapping for EEDR {
    fn get_reg_addr() -> *mut u8 {
        inner::EEDR
    }
}

pub struct EEARL;
impl RegisterMapping for EEARL {
    fn get_reg_addr() -> *mut u8 {
        inner::EEARL
    }
}

pub struct EEARH;
impl RegisterMapping for EEARH {
    fn get_reg_addr() -> *mut u8 {
        inner::EEARH
    }
}

pub struct GTCCR;
impl RegisterMapping for GTCCR {
    fn get_reg_addr() -> *mut u8 {
        inner::GTCCR
    }
}

pub struct TCCR0A;
impl RegisterMapping for TCCR0A {
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR0A
    }
}

pub struct TCCR0B;
impl RegisterMapping for TCCR0B {
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR0B
    }
}

pub struct TCNT0;
impl RegisterMapping for TCNT0 {
    fn get_reg_addr() -> *mut u8 {
        inner::TCNT0
    }
}

pub struct OCR0A;
impl RegisterMapping for OCR0A {
    fn get_reg_addr() -> *mut u8 {
        inner::OCR0A
    }
}

pub struct OCR0B;
impl RegisterMapping for OCR0B {
    fn get_reg_addr() -> *mut u8 {
        inner::OCR0B
    }
}

pub struct GPIOR1;
impl RegisterMapping for GPIOR1 {
    fn get_reg_addr() -> *mut u8 {
        inner::GPIOR1
    }
}

pub struct GPIOR2;
impl RegisterMapping for GPIOR2 {
    fn get_reg_addr() -> *mut u8 {
        inner::GPIOR2
    }
}

pub struct SPCR;
impl RegisterMapping for SPCR {
    fn get_reg_addr() -> *mut u8 {
        inner::SPCR
    }
}

pub struct SPSR;
impl RegisterMapping for SPSR {
    fn get_reg_addr() -> *mut u8 {
        inner::SPSR
    }
}

pub struct SPDR;
impl RegisterMapping for SPDR {
    fn get_reg_addr() -> *mut u8 {
        inner::SPDR
    }
}

pub struct ACSR;
impl RegisterMapping for ACSR {
    fn get_reg_addr() -> *mut u8 {
        inner::ACSR
    }
}

pub struct SMCR;
impl RegisterMapping for SMCR {
    fn get_reg_addr() -> *mut u8 {
        inner::SMCR
    }
}

pub struct MCUSR;
impl RegisterMapping for MCUSR {
    fn get_reg_addr() -> *mut u8 {
        inner::MCUSR
    }
}

pub struct MCUCR;
impl RegisterMapping for MCUCR {
    fn get_reg_addr() -> *mut u8 {
        inner::MCUCR
    }
}

pub struct SPMCSR;
impl RegisterMapping for SPMCSR {
    fn get_reg_addr() -> *mut u8 {
        inner::SPMCSR
    }
}

pub struct SPL;
impl RegisterMapping for SPL {
    fn get_reg_addr() -> *mut u8 {
        inner::SPL
    }
}

pub struct SPH;
impl RegisterMapping for SPH {
    fn get_reg_addr() -> *mut u8 {
        inner::SPH
    }
}

pub struct SREG;
impl RegisterMapping for SREG {
    fn get_reg_addr() -> *mut u8 {
        inner::SREG
    }
}

pub struct WDTCSR;
impl RegisterMapping for WDTCSR {
    fn get_reg_addr() -> *mut u8 {
        inner::WDTCSR
    }
}

pub struct CLKPR;
impl RegisterMapping for CLKPR {
    fn get_reg_addr() -> *mut u8 {
        inner::CLKPR
    }
}

pub struct PRR;
impl RegisterMapping for PRR {
    fn get_reg_addr() -> *mut u8 {
        inner::PRR
    }
}

pub struct OSCCAL;
impl RegisterMapping for OSCCAL {
    fn get_reg_addr() -> *mut u8 {
        inner::OSCCAL
    }
}

pub struct PCICR;
impl RegisterMapping for PCICR {
    fn get_reg_addr() -> *mut u8 {
        inner::PCICR
    }
}

pub struct EICRA;
impl RegisterMapping for EICRA {
    fn get_reg_addr() -> *mut u8 {
        inner::EICRA
    }
}

pub struct PCMSK0;
impl RegisterMapping for PCMSK0 {
    fn get_reg_addr() -> *mut u8 {
        inner::PCMSK0
    }
}

pub struct PCMSK1;
impl RegisterMapping for PCMSK1 {
    fn get_reg_addr() -> *mut u8 {
        inner::PCMSK1
    }
}

pub struct PCMSK2;
impl RegisterMapping for PCMSK2 {
    fn get_reg_addr() -> *mut u8 {
        inner::PCMSK2
    }
}

pub struct TIMSK0;
impl RegisterMapping for TIMSK0 {
    fn get_reg_addr() -> *mut u8 {
        inner::TIMSK0
    }
}

pub struct TIMSK1;
impl RegisterMapping for TIMSK1 {
    fn get_reg_addr() -> *mut u8 {
        inner::TIMSK1
    }
}

pub struct TIMSK2;
impl RegisterMapping for TIMSK2 {
    fn get_reg_addr() -> *mut u8 {
        inner::TIMSK2
    }
}

pub struct ADCL;
impl RegisterMapping for ADCL {
    fn get_reg_addr() -> *mut u8 {
        inner::ADCL
    }
}

pub struct ADCH;
impl RegisterMapping for ADCH {
    fn get_reg_addr() -> *mut u8 {
        inner::ADCH
    }
}

pub struct ADCSRA;
impl RegisterMapping for ADCSRA {
    fn get_reg_addr() -> *mut u8 {
        inner::ADCSRA
    }
}

pub struct ADCSRB;
impl RegisterMapping for ADCSRB {
    fn get_reg_addr() -> *mut u8 {
        inner::ADCSRB
    }
}

pub struct ADMUX;
impl RegisterMapping for ADMUX {
    fn get_reg_addr() -> *mut u8 {
        inner::ADMUX
    }
}

pub struct DIDR0;
impl RegisterMapping for DIDR0 {
    fn get_reg_addr() -> *mut u8 {
        inner::DIDR0
    }
}

pub struct DIDR1;
impl RegisterMapping for DIDR1 {
    fn get_reg_addr() -> *mut u8 {
        inner::DIDR1
    }
}

pub struct TCCR1A;
impl RegisterMapping for TCCR1A {
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR1A
    }
}

pub struct TCCR1B;
impl RegisterMapping for TCCR1B {
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR1B
    }
}

pub struct TCCR1C;
impl RegisterMapping for TCCR1C {
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR1C
    }
}

pub struct TCNT1L;
impl RegisterMapping for TCNT1L {
    fn get_reg_addr() -> *mut u8 {
        inner::TCNT1L
    }
}

pub struct TCNT1H;
impl RegisterMapping for TCNT1H {
    fn get_reg_addr() -> *mut u8 {
        inner::TCNT1H
    }
}

pub struct ICR1L;
impl RegisterMapping for ICR1L {
    fn get_reg_addr() -> *mut u8 {
        inner::ICR1L
    }
}

pub struct ICR1H;
impl RegisterMapping for ICR1H {
    fn get_reg_addr() -> *mut u8 {
        inner::ICR1H
    }
}

pub struct OCR1AL;
impl RegisterMapping for OCR1AL {
    fn get_reg_addr() -> *mut u8 {
        inner::OCR1AL
    }
}

pub struct OCR1AH;
impl RegisterMapping for OCR1AH {
    fn get_reg_addr() -> *mut u8 {
        inner::OCR1AH
    }
}

pub struct OCR1BL;
impl RegisterMapping for OCR1BL {
    fn get_reg_addr() -> *mut u8 {
        inner::OCR1BL
    }
}

pub struct OCR1BH;
impl RegisterMapping for OCR1BH {
    fn get_reg_addr() -> *mut u8 {
        inner::OCR1BH
    }
}

pub struct TCCR2A;
impl RegisterMapping for TCCR2A {
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR2A
    }
}

pub struct TCCR2B;
impl RegisterMapping for TCCR2B {
    fn get_reg_addr() -> *mut u8 {
        inner::TCCR2B
    }
}

pub struct TCNT2;
impl RegisterMapping for TCNT2 {
    fn get_reg_addr() -> *mut u8 {
        inner::TCNT2
    }
}

pub struct OCR2A;
impl RegisterMapping for OCR2A {
    fn get_reg_addr() -> *mut u8 {
        inner::OCR2A
    }
}

pub struct OCR2B;
impl RegisterMapping for OCR2B {
    fn get_reg_addr() -> *mut u8 {
        inner::OCR2B
    }
}

pub struct ASSR;
impl RegisterMapping for ASSR {
    fn get_reg_addr() -> *mut u8 {
        inner::ASSR
    }
}

pub struct TWBR;
impl RegisterMapping for TWBR {
    fn get_reg_addr() -> *mut u8 {
        inner::TWBR
    }
}

pub struct TWSR;
impl RegisterMapping for TWSR {
    fn get_reg_addr() -> *mut u8 {
        inner::TWSR
    }
}

pub struct TWAR;
impl RegisterMapping for TWAR {
    fn get_reg_addr() -> *mut u8 {
        inner::TWAR
    }
}

pub struct TWDR;
impl RegisterMapping for TWDR {
    fn get_reg_addr() -> *mut u8 {
        inner::TWDR
    }
}

pub struct TWCR;
impl RegisterMapping for TWCR {
    fn get_reg_addr() -> *mut u8 {
        inner::TWCR
    }
}

pub struct TWAMR;
impl RegisterMapping for TWAMR {
    fn get_reg_addr() -> *mut u8 {
        inner::TWAMR
    }
}

pub struct UCSR0A;
impl RegisterMapping for UCSR0A {
    fn get_reg_addr() -> *mut u8 {
        inner::UCSR0A
    }
}

pub struct UCSR0B;
impl RegisterMapping for UCSR0B {
    fn get_reg_addr() -> *mut u8 {
        inner::UCSR0B
    }
}

pub struct UCSR0C;
impl RegisterMapping for UCSR0C {
    fn get_reg_addr() -> *mut u8 {
        inner::UCSR0C
    }
}

pub struct UBRR0L;
impl RegisterMapping for UBRR0L {
    fn get_reg_addr() -> *mut u8 {
        inner::UBRR0L
    }
}

pub struct UBRR0H;
impl RegisterMapping for UBRR0H {
    fn get_reg_addr() -> *mut u8 {
        inner::UBRR0H
    }
}

pub struct UDR0;
impl RegisterMapping for UDR0 {
    fn get_reg_addr() -> *mut u8 {
        inner::UDR0
    }
}
