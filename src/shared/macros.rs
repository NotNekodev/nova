//=======================LOGGER MACROS======================
#[macro_export]
macro_rules! log {
    ($shared:expr,$level:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).log($level,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! popup {
    ($shared:expr,$level:expr,$title:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).popup($level,$title,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! trace {
    ($shared:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).log(simplelog::Level::Trace,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! debug {
    ($shared:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).log(simplelog::Level::Debug,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! info {
    ($shared:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).log(simplelog::Level::Info,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}


#[macro_export]
macro_rules! warn {
    ($shared:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).log(simplelog::Level::Warn,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! err {
    ($shared:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).log(simplelog::Level::Error,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! fatal {
    ($shared:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).fatal(format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! popup_info {
    ($shared:expr,$title:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).popup(simplelog::Level::Info,$title,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! popup_warn {
    ($shared:expr,$title:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).popup(simplelog::Level::Warn,$title,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! popup_err {
    ($shared:expr,$title:expr,$fmt:expr $(, $args:expr)* ) => {
        (*$shared.logger.lock().unwrap()).popup(simplelog::Level::Error,$title,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

//==============EARLY LOGGER MACROS===============

#[macro_export]
macro_rules! trace_early {
    ($logger:expr,$fmt:expr $(, $args:expr)* ) => {
        $logger.log(simplelog::Level::Trace,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! debug_early {
    ($logger:expr,$fmt:expr $(, $args:expr)* ) => {
        $logger.log(simplelog::Level::Debug,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! info_early {
    ($logger:expr,$fmt:expr $(, $args:expr)* ) => {
        $logger.log(simplelog::Level::Info,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}


#[macro_export]
macro_rules! warn_early {
    ($logger:expr,$fmt:expr $(, $args:expr)* ) => {
        $logger.log(simplelog::Level::Warn,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}

#[macro_export]
macro_rules! err_early {
    ($logger:expr,$fmt:expr $(, $args:expr)* ) => {
        $logger.log(simplelog::Level::Error,format_args!($fmt $(, $args)*),Option::Some(file!()),Option::Some(line!()))
    }
}