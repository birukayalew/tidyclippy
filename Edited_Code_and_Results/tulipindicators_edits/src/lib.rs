#![feature(extern_types)]
#![feature(label_break_value)]

pub mod benchmark;
pub mod cli;
pub mod fuzzer;
pub mod sample;
pub mod candles;
pub mod example1;
pub mod example2;
pub mod indicators_rs; 
pub mod smoke;
pub mod tiamalgamation;

pub mod beta {
    pub mod abands;
    pub mod alma;
    pub mod ce;
    pub mod cmf;
    pub mod copp;
    pub mod dc;
    pub mod fi;
    pub mod ikhts;
    pub mod kc;
    pub mod kst;
    pub mod mama;
    pub mod pbands;
    pub mod pc;
    pub mod pfe;
    pub mod posc;
    pub mod rmi;
    pub mod rmta;
    pub mod rvi;
    pub mod smi;
    pub mod tsi;
    pub mod vwap;
}

pub mod indicators {
    pub mod abs;
    pub mod acos;
    pub mod ad;
    pub mod add;
    pub mod adosc;
    pub mod adx;
    pub mod adxr;
    pub mod ao;
    pub mod apo;
    pub mod aroon;
    pub mod aroonosc;
    pub mod asin;
    pub mod atan;
    pub mod atr;
    pub mod avgprice;
    pub mod bbands;
    pub mod bop;
    pub mod cci;
    pub mod ceil;
    pub mod cmo;
    pub mod cos;
    pub mod cosh;
    pub mod crossany;
    pub mod crossover;
    pub mod cvi;
    pub mod decay;
    pub mod dema;
    pub mod di;
    pub mod div;
    pub mod dm;
    pub mod dpo;
    pub mod dx;
    pub mod edecay;
    pub mod ema;
    pub mod emv;
    pub mod exp;
    pub mod fisher;
    pub mod floor;
    pub mod fosc;
    pub mod hma;
    pub mod kama;
    pub mod kvo;
    pub mod lag;
    pub mod linreg;
    pub mod linregintercept;
    pub mod linregslope;
    pub mod ln;
    pub mod log10;
    pub mod macd;
    pub mod marketfi;
    pub mod mass;
    pub mod max;
    pub mod md;
    pub mod medprice;
    pub mod mfi;
    pub mod min;
    pub mod mom;
    pub mod msw;
    pub mod mul;
    pub mod natr;
    pub mod nvi;
    pub mod obv;
    pub mod ppo;
    pub mod psar;
    pub mod pvi;
    pub mod qstick;
    pub mod roc;
    pub mod rocr;
    pub mod round;
    pub mod rsi;
    pub mod sin;
    pub mod sinh;
    pub mod sma;
    pub mod sqrt;
    pub mod stddev;
    pub mod stderr;
    pub mod stoch;
    pub mod stochrsi;
    pub mod sub;
    pub mod sum;
    pub mod tan;
    pub mod tanh;
    pub mod tema;
    pub mod todeg;
    pub mod torad;
    pub mod tr;
    pub mod trima;
    pub mod trix;
    pub mod trunc;
    pub mod tsf;
    pub mod typprice;
    pub mod ultosc;
    pub mod var;
    pub mod vhf;
    pub mod vidya;
    pub mod volatility;
    pub mod vosc;
    pub mod vwma;
    pub mod wad;
    pub mod wcprice;
    pub mod wilders;
    pub mod willr;
    pub mod wma;
    pub mod zlema;
}

pub mod templates {
    pub mod candles;
    pub mod indicators;
}

pub mod utils {
    pub mod buffer;
}
