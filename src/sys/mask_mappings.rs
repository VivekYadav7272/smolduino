#![allow(non_camel_case_types)]
use super::reg_io::{MaskMapping, RegisterMapping};
use avrd::current as inner;

pub struct ACO;
impl MaskMapping for ACO {
    type Register = super::regs::ACSR;

    fn get_mask() -> u8 {
        inner::ACO as u8
    }
}

pub struct ACIS;
impl MaskMapping for ACIS {
    type Register = super::regs::ACSR;

    fn get_mask() -> u8 {
        inner::ACIS as u8
    }
}

pub struct ACD;
impl MaskMapping for ACD {
    type Register = super::regs::ACSR;

    fn get_mask() -> u8 {
        inner::ACD as u8
    }
}

pub struct ACIE;
impl MaskMapping for ACIE {
    type Register = super::regs::ACSR;

    fn get_mask() -> u8 {
        inner::ACIE as u8
    }
}

pub struct ADIE;
impl MaskMapping for ADIE {
    type Register = super::regs::ADCSRA;

    fn get_mask() -> u8 {
        inner::ADIE as u8
    }
}

pub struct ADSC;
impl MaskMapping for ADSC {
    type Register = super::regs::ADCSRA;

    fn get_mask() -> u8 {
        inner::ADSC as u8
    }
}

pub struct ADPS;
impl MaskMapping for ADPS {
    type Register = super::regs::ADCSRA;

    fn get_mask() -> u8 {
        inner::ADPS as u8
    }
}

pub struct ADIF;
impl MaskMapping for ADIF {
    type Register = super::regs::ADCSRA;

    fn get_mask() -> u8 {
        inner::ADIF as u8
    }
}

pub struct ADEN;
impl MaskMapping for ADEN {
    type Register = super::regs::ADCSRA;

    fn get_mask() -> u8 {
        inner::ADEN as u8
    }
}

pub struct ADATE;
impl MaskMapping for ADATE {
    type Register = super::regs::ADCSRA;

    fn get_mask() -> u8 {
        inner::ADATE as u8
    }
}

pub struct ACME;
impl MaskMapping for ACME {
    type Register = super::regs::ADCSRB;

    fn get_mask() -> u8 {
        inner::ACME as u8
    }
}

pub struct ADTS;
impl MaskMapping for ADTS {
    type Register = super::regs::ADCSRB;

    fn get_mask() -> u8 {
        inner::ADTS as u8
    }
}

pub struct REFS;
impl MaskMapping for REFS {
    type Register = super::regs::ADMUX;

    fn get_mask() -> u8 {
        inner::REFS as u8
    }
}

pub struct ADLAR;
impl MaskMapping for ADLAR {
    type Register = super::regs::ADMUX;

    fn get_mask() -> u8 {
        inner::ADLAR as u8
    }
}

pub struct MUX;
impl MaskMapping for MUX {
    type Register = super::regs::ADMUX;

    fn get_mask() -> u8 {
        inner::MUX as u8
    }
}

pub struct TCN2UB;
impl MaskMapping for TCN2UB {
    type Register = super::regs::ASSR;

    fn get_mask() -> u8 {
        inner::TCN2UB as u8
    }
}

pub struct OCR2BUB;
impl MaskMapping for OCR2BUB {
    type Register = super::regs::ASSR;

    fn get_mask() -> u8 {
        inner::OCR2BUB as u8
    }
}

pub struct AS2;
impl MaskMapping for AS2 {
    type Register = super::regs::ASSR;

    fn get_mask() -> u8 {
        inner::AS2 as u8
    }
}

pub struct EXCLK;
impl MaskMapping for EXCLK {
    type Register = super::regs::ASSR;

    fn get_mask() -> u8 {
        inner::EXCLK as u8
    }
}

pub struct OCR2AUB;
impl MaskMapping for OCR2AUB {
    type Register = super::regs::ASSR;

    fn get_mask() -> u8 {
        inner::OCR2AUB as u8
    }
}

pub struct TCR2AUB;
impl MaskMapping for TCR2AUB {
    type Register = super::regs::ASSR;

    fn get_mask() -> u8 {
        inner::TCR2AUB as u8
    }
}

pub struct TCR2BUB;
impl MaskMapping for TCR2BUB {
    type Register = super::regs::ASSR;

    fn get_mask() -> u8 {
        inner::TCR2BUB as u8
    }
}

pub struct CLKPCE;
impl MaskMapping for CLKPCE {
    type Register = super::regs::CLKPR;

    fn get_mask() -> u8 {
        inner::CLKPCE as u8
    }
}

pub struct CLKPS;
impl MaskMapping for CLKPS {
    type Register = super::regs::CLKPR;

    fn get_mask() -> u8 {
        inner::CLKPS as u8
    }
}

pub struct ADC0D;
impl MaskMapping for ADC0D {
    type Register = super::regs::DIDR0;

    fn get_mask() -> u8 {
        inner::ADC0D as u8
    }
}

pub struct ADC2D;
impl MaskMapping for ADC2D {
    type Register = super::regs::DIDR0;

    fn get_mask() -> u8 {
        inner::ADC2D as u8
    }
}

pub struct ADC4D;
impl MaskMapping for ADC4D {
    type Register = super::regs::DIDR0;

    fn get_mask() -> u8 {
        inner::ADC4D as u8
    }
}

pub struct ADC5D;
impl MaskMapping for ADC5D {
    type Register = super::regs::DIDR0;

    fn get_mask() -> u8 {
        inner::ADC5D as u8
    }
}

pub struct ADC1D;
impl MaskMapping for ADC1D {
    type Register = super::regs::DIDR0;

    fn get_mask() -> u8 {
        inner::ADC1D as u8
    }
}

pub struct ADC3D;
impl MaskMapping for ADC3D {
    type Register = super::regs::DIDR0;

    fn get_mask() -> u8 {
        inner::ADC3D as u8
    }
}

pub struct AIN1D;
impl MaskMapping for AIN1D {
    type Register = super::regs::DIDR1;

    fn get_mask() -> u8 {
        inner::AIN1D as u8
    }
}

pub struct AIN0D;
impl MaskMapping for AIN0D {
    type Register = super::regs::DIDR1;

    fn get_mask() -> u8 {
        inner::AIN0D as u8
    }
}

pub struct EEMPE;
impl MaskMapping for EEMPE {
    type Register = super::regs::EECR;

    fn get_mask() -> u8 {
        inner::EEMPE as u8
    }
}

pub struct EEPM;
impl MaskMapping for EEPM {
    type Register = super::regs::EECR;

    fn get_mask() -> u8 {
        inner::EEPM as u8
    }
}

pub struct EERIE;
impl MaskMapping for EERIE {
    type Register = super::regs::EECR;

    fn get_mask() -> u8 {
        inner::EERIE as u8
    }
}

pub struct EERE;
impl MaskMapping for EERE {
    type Register = super::regs::EECR;

    fn get_mask() -> u8 {
        inner::EERE as u8
    }
}

pub struct EEPE;
impl MaskMapping for EEPE {
    type Register = super::regs::EECR;

    fn get_mask() -> u8 {
        inner::EEPE as u8
    }
}

pub struct ISC1;
impl MaskMapping for ISC1 {
    type Register = super::regs::EICRA;

    fn get_mask() -> u8 {
        inner::ISC1 as u8
    }
}

pub struct ISC0;
impl MaskMapping for ISC0 {
    type Register = super::regs::EICRA;

    fn get_mask() -> u8 {
        inner::ISC0 as u8
    }
}

pub struct INTF;
impl MaskMapping for INTF {
    type Register = super::regs::EIFR;

    fn get_mask() -> u8 {
        inner::INTF as u8
    }
}

pub struct INT;
impl MaskMapping for INT {
    type Register = super::regs::EIMSK;

    fn get_mask() -> u8 {
        inner::INT as u8
    }
}

pub struct BODLEVEL;
impl MaskMapping for BODLEVEL {
    type Register = super::regs::EXTENDED;

    fn get_mask() -> u8 {
        inner::BODLEVEL as u8
    }
}

pub struct TSM;
impl MaskMapping for TSM {
    type Register = super::regs::GTCCR;

    fn get_mask() -> u8 {
        inner::TSM as u8
    }
}

pub struct PSRSYNC;
impl MaskMapping for PSRSYNC {
    type Register = super::regs::GTCCR;

    fn get_mask() -> u8 {
        inner::PSRSYNC as u8
    }
}

pub struct BOOTRST;
impl MaskMapping for BOOTRST {
    type Register = super::regs::HIGH;

    fn get_mask() -> u8 {
        inner::BOOTRST as u8
    }
}

pub struct BOOTSZ;
impl MaskMapping for BOOTSZ {
    type Register = super::regs::HIGH;

    fn get_mask() -> u8 {
        inner::BOOTSZ as u8
    }
}

pub struct DWEN;
impl MaskMapping for DWEN {
    type Register = super::regs::HIGH;

    fn get_mask() -> u8 {
        inner::DWEN as u8
    }
}

pub struct WDTON;
impl MaskMapping for WDTON {
    type Register = super::regs::HIGH;

    fn get_mask() -> u8 {
        inner::WDTON as u8
    }
}

pub struct RSTDISBL;
impl MaskMapping for RSTDISBL {
    type Register = super::regs::HIGH;

    fn get_mask() -> u8 {
        inner::RSTDISBL as u8
    }
}

pub struct EESAVE;
impl MaskMapping for EESAVE {
    type Register = super::regs::HIGH;

    fn get_mask() -> u8 {
        inner::EESAVE as u8
    }
}

pub struct SPIEN;
impl MaskMapping for SPIEN {
    type Register = super::regs::HIGH;

    fn get_mask() -> u8 {
        inner::SPIEN as u8
    }
}

pub struct LB;
impl MaskMapping for LB {
    type Register = super::regs::LOCKBIT;

    fn get_mask() -> u8 {
        inner::LB as u8
    }
}

pub struct BLB0;
impl MaskMapping for BLB0 {
    type Register = super::regs::LOCKBIT;

    fn get_mask() -> u8 {
        inner::BLB0 as u8
    }
}

pub struct BLB1;
impl MaskMapping for BLB1 {
    type Register = super::regs::LOCKBIT;

    fn get_mask() -> u8 {
        inner::BLB1 as u8
    }
}

pub struct SUT_CKSEL;
impl MaskMapping for SUT_CKSEL {
    type Register = super::regs::LOW;

    fn get_mask() -> u8 {
        inner::SUT_CKSEL as u8
    }
}

pub struct CKDIV8;
impl MaskMapping for CKDIV8 {
    type Register = super::regs::LOW;

    fn get_mask() -> u8 {
        inner::CKDIV8 as u8
    }
}

pub struct CKOUT;
impl MaskMapping for CKOUT {
    type Register = super::regs::LOW;

    fn get_mask() -> u8 {
        inner::CKOUT as u8
    }
}

pub struct BODSE;
impl MaskMapping for BODSE {
    type Register = super::regs::MCUCR;

    fn get_mask() -> u8 {
        inner::BODSE as u8
    }
}

pub struct IVSEL;
impl MaskMapping for IVSEL {
    type Register = super::regs::MCUCR;

    fn get_mask() -> u8 {
        inner::IVSEL as u8
    }
}

pub struct PUD;
impl MaskMapping for PUD {
    type Register = super::regs::MCUCR;

    fn get_mask() -> u8 {
        inner::PUD as u8
    }
}

pub struct IVCE;
impl MaskMapping for IVCE {
    type Register = super::regs::MCUCR;

    fn get_mask() -> u8 {
        inner::IVCE as u8
    }
}

pub struct BODS;
impl MaskMapping for BODS {
    type Register = super::regs::MCUCR;

    fn get_mask() -> u8 {
        inner::BODS as u8
    }
}

pub struct EXTRF;
impl MaskMapping for EXTRF {
    type Register = super::regs::MCUSR;

    fn get_mask() -> u8 {
        inner::EXTRF as u8
    }
}

pub struct WDRF;
impl MaskMapping for WDRF {
    type Register = super::regs::MCUSR;

    fn get_mask() -> u8 {
        inner::WDRF as u8
    }
}

pub struct PORF;
impl MaskMapping for PORF {
    type Register = super::regs::MCUSR;

    fn get_mask() -> u8 {
        inner::PORF as u8
    }
}

pub struct BORF;
impl MaskMapping for BORF {
    type Register = super::regs::MCUSR;

    fn get_mask() -> u8 {
        inner::BORF as u8
    }
}

pub struct PCIE;
impl MaskMapping for PCIE {
    type Register = super::regs::PCICR;

    fn get_mask() -> u8 {
        inner::PCIE as u8
    }
}

pub struct PCIF;
impl MaskMapping for PCIF {
    type Register = super::regs::PCIFR;

    fn get_mask() -> u8 {
        inner::PCIF as u8
    }
}

pub struct PRTIM2;
impl MaskMapping for PRTIM2 {
    type Register = super::regs::PRR;

    fn get_mask() -> u8 {
        inner::PRTIM2 as u8
    }
}

pub struct PRTIM0;
impl MaskMapping for PRTIM0 {
    type Register = super::regs::PRR;

    fn get_mask() -> u8 {
        inner::PRTIM0 as u8
    }
}

pub struct PRSPI;
impl MaskMapping for PRSPI {
    type Register = super::regs::PRR;

    fn get_mask() -> u8 {
        inner::PRSPI as u8
    }
}

pub struct PRADC;
impl MaskMapping for PRADC {
    type Register = super::regs::PRR;

    fn get_mask() -> u8 {
        inner::PRADC as u8
    }
}

pub struct PRTIM1;
impl MaskMapping for PRTIM1 {
    type Register = super::regs::PRR;

    fn get_mask() -> u8 {
        inner::PRTIM1 as u8
    }
}

pub struct PRTWI;
impl MaskMapping for PRTWI {
    type Register = super::regs::PRR;

    fn get_mask() -> u8 {
        inner::PRTWI as u8
    }
}

pub struct PRUSART0;
impl MaskMapping for PRUSART0 {
    type Register = super::regs::PRR;

    fn get_mask() -> u8 {
        inner::PRUSART0 as u8
    }
}

pub struct SM;
impl MaskMapping for SM {
    type Register = super::regs::SMCR;

    fn get_mask() -> u8 {
        inner::SM as u8
    }
}

pub struct SE;
impl MaskMapping for SE {
    type Register = super::regs::SMCR;

    fn get_mask() -> u8 {
        inner::SE as u8
    }
}

pub struct SPIE;
impl MaskMapping for SPIE {
    type Register = super::regs::SPCR;

    fn get_mask() -> u8 {
        inner::SPIE as u8
    }
}

pub struct DORD;
impl MaskMapping for DORD {
    type Register = super::regs::SPCR;

    fn get_mask() -> u8 {
        inner::DORD as u8
    }
}

pub struct SPR;
impl MaskMapping for SPR {
    type Register = super::regs::SPCR;

    fn get_mask() -> u8 {
        inner::SPR as u8
    }
}

pub struct CPHA;
impl MaskMapping for CPHA {
    type Register = super::regs::SPCR;

    fn get_mask() -> u8 {
        inner::CPHA as u8
    }
}

pub struct MSTR;
impl MaskMapping for MSTR {
    type Register = super::regs::SPCR;

    fn get_mask() -> u8 {
        inner::MSTR as u8
    }
}

pub struct SPE;
impl MaskMapping for SPE {
    type Register = super::regs::SPCR;

    fn get_mask() -> u8 {
        inner::SPE as u8
    }
}

pub struct CPOL;
impl MaskMapping for CPOL {
    type Register = super::regs::SPCR;

    fn get_mask() -> u8 {
        inner::CPOL as u8
    }
}

pub struct PGWRT;
impl MaskMapping for PGWRT {
    type Register = super::regs::SPMCSR;

    fn get_mask() -> u8 {
        inner::PGWRT as u8
    }
}

pub struct SIGRD;
impl MaskMapping for SIGRD {
    type Register = super::regs::SPMCSR;

    fn get_mask() -> u8 {
        inner::SIGRD as u8
    }
}

pub struct BLBSET;
impl MaskMapping for BLBSET {
    type Register = super::regs::SPMCSR;

    fn get_mask() -> u8 {
        inner::BLBSET as u8
    }
}

pub struct RWWSB;
impl MaskMapping for RWWSB {
    type Register = super::regs::SPMCSR;

    fn get_mask() -> u8 {
        inner::RWWSB as u8
    }
}

pub struct SPMIE;
impl MaskMapping for SPMIE {
    type Register = super::regs::SPMCSR;

    fn get_mask() -> u8 {
        inner::SPMIE as u8
    }
}

pub struct SPMEN;
impl MaskMapping for SPMEN {
    type Register = super::regs::SPMCSR;

    fn get_mask() -> u8 {
        inner::SPMEN as u8
    }
}

pub struct RWWSRE;
impl MaskMapping for RWWSRE {
    type Register = super::regs::SPMCSR;

    fn get_mask() -> u8 {
        inner::RWWSRE as u8
    }
}

pub struct PGERS;
impl MaskMapping for PGERS {
    type Register = super::regs::SPMCSR;

    fn get_mask() -> u8 {
        inner::PGERS as u8
    }
}

pub struct WCOL;
impl MaskMapping for WCOL {
    type Register = super::regs::SPSR;

    fn get_mask() -> u8 {
        inner::WCOL as u8
    }
}

pub struct SPI2X;
impl MaskMapping for SPI2X {
    type Register = super::regs::SPSR;

    fn get_mask() -> u8 {
        inner::SPI2X as u8
    }
}

pub struct SPIF;
impl MaskMapping for SPIF {
    type Register = super::regs::SPSR;

    fn get_mask() -> u8 {
        inner::SPIF as u8
    }
}

pub struct V;
impl MaskMapping for V {
    type Register = super::regs::SREG;

    fn get_mask() -> u8 {
        inner::V as u8
    }
}

pub struct T;
impl MaskMapping for T {
    type Register = super::regs::SREG;

    fn get_mask() -> u8 {
        inner::T as u8
    }
}

pub struct S;
impl MaskMapping for S {
    type Register = super::regs::SREG;

    fn get_mask() -> u8 {
        inner::S as u8
    }
}

pub struct N;
impl MaskMapping for N {
    type Register = super::regs::SREG;

    fn get_mask() -> u8 {
        inner::N as u8
    }
}

pub struct I;
impl MaskMapping for I {
    type Register = super::regs::SREG;

    fn get_mask() -> u8 {
        inner::I as u8
    }
}

pub struct C;
impl MaskMapping for C {
    type Register = super::regs::SREG;

    fn get_mask() -> u8 {
        inner::C as u8
    }
}

pub struct H;
impl MaskMapping for H {
    type Register = super::regs::SREG;

    fn get_mask() -> u8 {
        inner::H as u8
    }
}

pub struct Z;
impl MaskMapping for Z {
    type Register = super::regs::SREG;

    fn get_mask() -> u8 {
        inner::Z as u8
    }
}

pub struct COM0A;
impl MaskMapping for COM0A {
    type Register = super::regs::TCCR0A;

    fn get_mask() -> u8 {
        inner::COM0A as u8
    }
}

pub struct WGM0;
impl MaskMapping for WGM0 {
    type Register = super::regs::TCCR0A;

    fn get_mask() -> u8 {
        inner::WGM0 as u8
    }
}

pub struct COM0B;
impl MaskMapping for COM0B {
    type Register = super::regs::TCCR0A;

    fn get_mask() -> u8 {
        inner::COM0B as u8
    }
}

pub struct CS0;
impl MaskMapping for CS0 {
    type Register = super::regs::TCCR0B;

    fn get_mask() -> u8 {
        inner::CS0 as u8
    }
}

pub struct WGM02;
impl MaskMapping for WGM02 {
    type Register = super::regs::TCCR0B;

    fn get_mask() -> u8 {
        inner::WGM02 as u8
    }
}

pub struct FOC0B;
impl MaskMapping for FOC0B {
    type Register = super::regs::TCCR0B;

    fn get_mask() -> u8 {
        inner::FOC0B as u8
    }
}

pub struct FOC0A;
impl MaskMapping for FOC0A {
    type Register = super::regs::TCCR0B;

    fn get_mask() -> u8 {
        inner::FOC0A as u8
    }
}

pub struct COM1A;
impl MaskMapping for COM1A {
    type Register = super::regs::TCCR1A;

    fn get_mask() -> u8 {
        inner::COM1A as u8
    }
}

pub struct COM1B;
impl MaskMapping for COM1B {
    type Register = super::regs::TCCR1A;

    fn get_mask() -> u8 {
        inner::COM1B as u8
    }
}

pub struct ICES1;
impl MaskMapping for ICES1 {
    type Register = super::regs::TCCR1B;

    fn get_mask() -> u8 {
        inner::ICES1 as u8
    }
}

pub struct ICNC1;
impl MaskMapping for ICNC1 {
    type Register = super::regs::TCCR1B;

    fn get_mask() -> u8 {
        inner::ICNC1 as u8
    }
}

pub struct CS1;
impl MaskMapping for CS1 {
    type Register = super::regs::TCCR1B;

    fn get_mask() -> u8 {
        inner::CS1 as u8
    }
}

pub struct FOC1A;
impl MaskMapping for FOC1A {
    type Register = super::regs::TCCR1C;

    fn get_mask() -> u8 {
        inner::FOC1A as u8
    }
}

pub struct FOC1B;
impl MaskMapping for FOC1B {
    type Register = super::regs::TCCR1C;

    fn get_mask() -> u8 {
        inner::FOC1B as u8
    }
}

pub struct COM2B;
impl MaskMapping for COM2B {
    type Register = super::regs::TCCR2A;

    fn get_mask() -> u8 {
        inner::COM2B as u8
    }
}

pub struct WGM2;
impl MaskMapping for WGM2 {
    type Register = super::regs::TCCR2A;

    fn get_mask() -> u8 {
        inner::WGM2 as u8
    }
}

pub struct COM2A;
impl MaskMapping for COM2A {
    type Register = super::regs::TCCR2A;

    fn get_mask() -> u8 {
        inner::COM2A as u8
    }
}

pub struct FOC2B;
impl MaskMapping for FOC2B {
    type Register = super::regs::TCCR2B;

    fn get_mask() -> u8 {
        inner::FOC2B as u8
    }
}

pub struct WGM22;
impl MaskMapping for WGM22 {
    type Register = super::regs::TCCR2B;

    fn get_mask() -> u8 {
        inner::WGM22 as u8
    }
}

pub struct CS2;
impl MaskMapping for CS2 {
    type Register = super::regs::TCCR2B;

    fn get_mask() -> u8 {
        inner::CS2 as u8
    }
}

pub struct FOC2A;
impl MaskMapping for FOC2A {
    type Register = super::regs::TCCR2B;

    fn get_mask() -> u8 {
        inner::FOC2A as u8
    }
}

pub struct OCF0A;
impl MaskMapping for OCF0A {
    type Register = super::regs::TIFR0;

    fn get_mask() -> u8 {
        inner::OCF0A as u8
    }
}

pub struct OCF0B;
impl MaskMapping for OCF0B {
    type Register = super::regs::TIFR0;

    fn get_mask() -> u8 {
        inner::OCF0B as u8
    }
}

pub struct TOV0;
impl MaskMapping for TOV0 {
    type Register = super::regs::TIFR0;

    fn get_mask() -> u8 {
        inner::TOV0 as u8
    }
}

pub struct OCF1A;
impl MaskMapping for OCF1A {
    type Register = super::regs::TIFR1;

    fn get_mask() -> u8 {
        inner::OCF1A as u8
    }
}

pub struct TOV1;
impl MaskMapping for TOV1 {
    type Register = super::regs::TIFR1;

    fn get_mask() -> u8 {
        inner::TOV1 as u8
    }
}

pub struct ICF1;
impl MaskMapping for ICF1 {
    type Register = super::regs::TIFR1;

    fn get_mask() -> u8 {
        inner::ICF1 as u8
    }
}

pub struct OCF1B;
impl MaskMapping for OCF1B {
    type Register = super::regs::TIFR1;

    fn get_mask() -> u8 {
        inner::OCF1B as u8
    }
}

pub struct OCF2B;
impl MaskMapping for OCF2B {
    type Register = super::regs::TIFR2;

    fn get_mask() -> u8 {
        inner::OCF2B as u8
    }
}

pub struct TOV2;
impl MaskMapping for TOV2 {
    type Register = super::regs::TIFR2;

    fn get_mask() -> u8 {
        inner::TOV2 as u8
    }
}

pub struct OCF2A;
impl MaskMapping for OCF2A {
    type Register = super::regs::TIFR2;

    fn get_mask() -> u8 {
        inner::OCF2A as u8
    }
}

pub struct OCIE0A;
impl MaskMapping for OCIE0A {
    type Register = super::regs::TIMSK0;

    fn get_mask() -> u8 {
        inner::OCIE0A as u8
    }
}

pub struct TOIE0;
impl MaskMapping for TOIE0 {
    type Register = super::regs::TIMSK0;

    fn get_mask() -> u8 {
        inner::TOIE0 as u8
    }
}

pub struct OCIE0B;
impl MaskMapping for OCIE0B {
    type Register = super::regs::TIMSK0;

    fn get_mask() -> u8 {
        inner::OCIE0B as u8
    }
}

pub struct OCIE1B;
impl MaskMapping for OCIE1B {
    type Register = super::regs::TIMSK1;

    fn get_mask() -> u8 {
        inner::OCIE1B as u8
    }
}

pub struct TOIE1;
impl MaskMapping for TOIE1 {
    type Register = super::regs::TIMSK1;

    fn get_mask() -> u8 {
        inner::TOIE1 as u8
    }
}

pub struct ICIE1;
impl MaskMapping for ICIE1 {
    type Register = super::regs::TIMSK1;

    fn get_mask() -> u8 {
        inner::ICIE1 as u8
    }
}

pub struct OCIE1A;
impl MaskMapping for OCIE1A {
    type Register = super::regs::TIMSK1;

    fn get_mask() -> u8 {
        inner::OCIE1A as u8
    }
}

pub struct TOIE2;
impl MaskMapping for TOIE2 {
    type Register = super::regs::TIMSK2;

    fn get_mask() -> u8 {
        inner::TOIE2 as u8
    }
}

pub struct OCIE2A;
impl MaskMapping for OCIE2A {
    type Register = super::regs::TIMSK2;

    fn get_mask() -> u8 {
        inner::OCIE2A as u8
    }
}

pub struct OCIE2B;
impl MaskMapping for OCIE2B {
    type Register = super::regs::TIMSK2;

    fn get_mask() -> u8 {
        inner::OCIE2B as u8
    }
}

pub struct TWAM;
impl MaskMapping for TWAM {
    type Register = super::regs::TWAMR;

    fn get_mask() -> u8 {
        inner::TWAM as u8
    }
}

pub struct TWA;
impl MaskMapping for TWA {
    type Register = super::regs::TWAR;

    fn get_mask() -> u8 {
        inner::TWA as u8
    }
}

pub struct TWGCE;
impl MaskMapping for TWGCE {
    type Register = super::regs::TWAR;

    fn get_mask() -> u8 {
        inner::TWGCE as u8
    }
}

pub struct TWINT;
impl MaskMapping for TWINT {
    type Register = super::regs::TWCR;

    fn get_mask() -> u8 {
        inner::TWINT as u8
    }
}

pub struct TWWC;
impl MaskMapping for TWWC {
    type Register = super::regs::TWCR;

    fn get_mask() -> u8 {
        inner::TWWC as u8
    }
}

pub struct TWEN;
impl MaskMapping for TWEN {
    type Register = super::regs::TWCR;

    fn get_mask() -> u8 {
        inner::TWEN as u8
    }
}

pub struct TWSTA;
impl MaskMapping for TWSTA {
    type Register = super::regs::TWCR;

    fn get_mask() -> u8 {
        inner::TWSTA as u8
    }
}

pub struct TWSTO;
impl MaskMapping for TWSTO {
    type Register = super::regs::TWCR;

    fn get_mask() -> u8 {
        inner::TWSTO as u8
    }
}

pub struct TWIE;
impl MaskMapping for TWIE {
    type Register = super::regs::TWCR;

    fn get_mask() -> u8 {
        inner::TWIE as u8
    }
}

pub struct TWEA;
impl MaskMapping for TWEA {
    type Register = super::regs::TWCR;

    fn get_mask() -> u8 {
        inner::TWEA as u8
    }
}

pub struct TWS;
impl MaskMapping for TWS {
    type Register = super::regs::TWSR;

    fn get_mask() -> u8 {
        inner::TWS as u8
    }
}

pub struct TWPS;
impl MaskMapping for TWPS {
    type Register = super::regs::TWSR;

    fn get_mask() -> u8 {
        inner::TWPS as u8
    }
}

pub struct DOR0;
impl MaskMapping for DOR0 {
    type Register = super::regs::UCSR0A;

    fn get_mask() -> u8 {
        inner::DOR0 as u8
    }
}

pub struct U2X0;
impl MaskMapping for U2X0 {
    type Register = super::regs::UCSR0A;

    fn get_mask() -> u8 {
        inner::U2X0 as u8
    }
}

pub struct RXC0;
impl MaskMapping for RXC0 {
    type Register = super::regs::UCSR0A;

    fn get_mask() -> u8 {
        inner::RXC0 as u8
    }
}

pub struct MPCM0;
impl MaskMapping for MPCM0 {
    type Register = super::regs::UCSR0A;

    fn get_mask() -> u8 {
        inner::MPCM0 as u8
    }
}

pub struct TXC0;
impl MaskMapping for TXC0 {
    type Register = super::regs::UCSR0A;

    fn get_mask() -> u8 {
        inner::TXC0 as u8
    }
}

pub struct UDRE0;
impl MaskMapping for UDRE0 {
    type Register = super::regs::UCSR0A;

    fn get_mask() -> u8 {
        inner::UDRE0 as u8
    }
}

pub struct FE0;
impl MaskMapping for FE0 {
    type Register = super::regs::UCSR0A;

    fn get_mask() -> u8 {
        inner::FE0 as u8
    }
}

pub struct UPE0;
impl MaskMapping for UPE0 {
    type Register = super::regs::UCSR0A;

    fn get_mask() -> u8 {
        inner::UPE0 as u8
    }
}

pub struct TXB80;
impl MaskMapping for TXB80 {
    type Register = super::regs::UCSR0B;

    fn get_mask() -> u8 {
        inner::TXB80 as u8
    }
}

pub struct TXEN0;
impl MaskMapping for TXEN0 {
    type Register = super::regs::UCSR0B;

    fn get_mask() -> u8 {
        inner::TXEN0 as u8
    }
}

pub struct RXB80;
impl MaskMapping for RXB80 {
    type Register = super::regs::UCSR0B;

    fn get_mask() -> u8 {
        inner::RXB80 as u8
    }
}

pub struct RXEN0;
impl MaskMapping for RXEN0 {
    type Register = super::regs::UCSR0B;

    fn get_mask() -> u8 {
        inner::RXEN0 as u8
    }
}

pub struct TXCIE0;
impl MaskMapping for TXCIE0 {
    type Register = super::regs::UCSR0B;

    fn get_mask() -> u8 {
        inner::TXCIE0 as u8
    }
}

pub struct RXCIE0;
impl MaskMapping for RXCIE0 {
    type Register = super::regs::UCSR0B;

    fn get_mask() -> u8 {
        inner::RXCIE0 as u8
    }
}

pub struct UDRIE0;
impl MaskMapping for UDRIE0 {
    type Register = super::regs::UCSR0B;

    fn get_mask() -> u8 {
        inner::UDRIE0 as u8
    }
}

pub struct UCSZ02;
impl MaskMapping for UCSZ02 {
    type Register = super::regs::UCSR0B;

    fn get_mask() -> u8 {
        inner::UCSZ02 as u8
    }
}

pub struct UCSZ0;
impl MaskMapping for UCSZ0 {
    type Register = super::regs::UCSR0C;

    fn get_mask() -> u8 {
        inner::UCSZ0 as u8
    }
}

pub struct USBS0;
impl MaskMapping for USBS0 {
    type Register = super::regs::UCSR0C;

    fn get_mask() -> u8 {
        inner::USBS0 as u8
    }
}

pub struct UCPOL0;
impl MaskMapping for UCPOL0 {
    type Register = super::regs::UCSR0C;

    fn get_mask() -> u8 {
        inner::UCPOL0 as u8
    }
}

pub struct UMSEL0;
impl MaskMapping for UMSEL0 {
    type Register = super::regs::UCSR0C;

    fn get_mask() -> u8 {
        inner::UMSEL0 as u8
    }
}

pub struct UPM0;
impl MaskMapping for UPM0 {
    type Register = super::regs::UCSR0C;

    fn get_mask() -> u8 {
        inner::UPM0 as u8
    }
}

pub struct WDCE;
impl MaskMapping for WDCE {
    type Register = super::regs::WDTCSR;

    fn get_mask() -> u8 {
        inner::WDCE as u8
    }
}

pub struct WDIE;
impl MaskMapping for WDIE {
    type Register = super::regs::WDTCSR;

    fn get_mask() -> u8 {
        inner::WDIE as u8
    }
}

pub struct WDE;
impl MaskMapping for WDE {
    type Register = super::regs::WDTCSR;

    fn get_mask() -> u8 {
        inner::WDE as u8
    }
}

pub struct WDIF;
impl MaskMapping for WDIF {
    type Register = super::regs::WDTCSR;

    fn get_mask() -> u8 {
        inner::WDIF as u8
    }
}

pub struct WDP;
impl MaskMapping for WDP {
    type Register = super::regs::WDTCSR;

    fn get_mask() -> u8 {
        inner::WDP as u8
    }
}
