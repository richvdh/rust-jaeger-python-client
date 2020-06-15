// Autogenerated by Thrift Compiler (0.12.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments, clippy::type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate ordered_float;
extern crate thrift;
extern crate try_from;

use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::From;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use try_from::TryFrom;

use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::protocol::{
    TFieldIdentifier, TInputProtocol, TListIdentifier, TMapIdentifier, TMessageIdentifier,
    TMessageType, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType,
};
use thrift::server::TProcessor;
use thrift::{
    ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient,
};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SamplingStrategyType {
    Probabilistic = 0,
    RateLimiting = 1,
}

impl SamplingStrategyType {
    pub fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
        o_prot.write_i32(*self as i32)
    }
    pub fn read_from_in_protocol(
        i_prot: &mut dyn TInputProtocol,
    ) -> thrift::Result<SamplingStrategyType> {
        let enum_value = i_prot.read_i32()?;
        SamplingStrategyType::try_from(enum_value)
    }
}

impl TryFrom<i32> for SamplingStrategyType {
    type Err = thrift::Error;
    fn try_from(i: i32) -> Result<Self, Self::Err> {
        match i {
            0 => Ok(SamplingStrategyType::Probabilistic),
            1 => Ok(SamplingStrategyType::RateLimiting),
            _ => Err(thrift::Error::Protocol(ProtocolError::new(
                ProtocolErrorKind::InvalidData,
                format!("cannot convert enum constant {} to SamplingStrategyType", i),
            ))),
        }
    }
}

//
// ProbabilisticSamplingStrategy
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProbabilisticSamplingStrategy {
    pub sampling_rate: OrderedFloat<f64>,
}

impl ProbabilisticSamplingStrategy {
    pub fn new(sampling_rate: OrderedFloat<f64>) -> ProbabilisticSamplingStrategy {
        ProbabilisticSamplingStrategy { sampling_rate }
    }
    pub fn read_from_in_protocol(
        i_prot: &mut dyn TInputProtocol,
    ) -> thrift::Result<ProbabilisticSamplingStrategy> {
        i_prot.read_struct_begin()?;
        let mut f_1: Option<OrderedFloat<f64>> = None;
        loop {
            let field_ident = i_prot.read_field_begin()?;
            if field_ident.field_type == TType::Stop {
                break;
            }
            let field_id = field_id(&field_ident)?;
            match field_id {
                1 => {
                    let val = OrderedFloat::from(i_prot.read_double()?);
                    f_1 = Some(val);
                }
                _ => {
                    i_prot.skip(field_ident.field_type)?;
                }
            };
            i_prot.read_field_end()?;
        }
        i_prot.read_struct_end()?;
        verify_required_field_exists("ProbabilisticSamplingStrategy.sampling_rate", &f_1)?;
        let ret = ProbabilisticSamplingStrategy {
            sampling_rate: f_1
                .expect("auto-generated code should have checked for presence of required fields"),
        };
        Ok(ret)
    }
    pub fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
        let struct_ident = TStructIdentifier::new("ProbabilisticSamplingStrategy");
        o_prot.write_struct_begin(&struct_ident)?;
        o_prot.write_field_begin(&TFieldIdentifier::new("samplingRate", TType::Double, 1))?;
        o_prot.write_double(self.sampling_rate.into())?;
        o_prot.write_field_end()?;
        o_prot.write_field_stop()?;
        o_prot.write_struct_end()
    }
}

//
// RateLimitingSamplingStrategy
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RateLimitingSamplingStrategy {
    pub max_traces_per_second: i16,
}

impl RateLimitingSamplingStrategy {
    pub fn new(max_traces_per_second: i16) -> RateLimitingSamplingStrategy {
        RateLimitingSamplingStrategy {
            max_traces_per_second,
        }
    }
    pub fn read_from_in_protocol(
        i_prot: &mut dyn TInputProtocol,
    ) -> thrift::Result<RateLimitingSamplingStrategy> {
        i_prot.read_struct_begin()?;
        let mut f_1: Option<i16> = None;
        loop {
            let field_ident = i_prot.read_field_begin()?;
            if field_ident.field_type == TType::Stop {
                break;
            }
            let field_id = field_id(&field_ident)?;
            match field_id {
                1 => {
                    let val = i_prot.read_i16()?;
                    f_1 = Some(val);
                }
                _ => {
                    i_prot.skip(field_ident.field_type)?;
                }
            };
            i_prot.read_field_end()?;
        }
        i_prot.read_struct_end()?;
        verify_required_field_exists("RateLimitingSamplingStrategy.max_traces_per_second", &f_1)?;
        let ret = RateLimitingSamplingStrategy {
            max_traces_per_second: f_1
                .expect("auto-generated code should have checked for presence of required fields"),
        };
        Ok(ret)
    }
    pub fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
        let struct_ident = TStructIdentifier::new("RateLimitingSamplingStrategy");
        o_prot.write_struct_begin(&struct_ident)?;
        o_prot.write_field_begin(&TFieldIdentifier::new("maxTracesPerSecond", TType::I16, 1))?;
        o_prot.write_i16(self.max_traces_per_second)?;
        o_prot.write_field_end()?;
        o_prot.write_field_stop()?;
        o_prot.write_struct_end()
    }
}

//
// OperationSamplingStrategy
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OperationSamplingStrategy {
    pub operation: String,
    pub probabilistic_sampling: ProbabilisticSamplingStrategy,
}

impl OperationSamplingStrategy {
    pub fn new(
        operation: String,
        probabilistic_sampling: ProbabilisticSamplingStrategy,
    ) -> OperationSamplingStrategy {
        OperationSamplingStrategy {
            operation,
            probabilistic_sampling,
        }
    }
    pub fn read_from_in_protocol(
        i_prot: &mut dyn TInputProtocol,
    ) -> thrift::Result<OperationSamplingStrategy> {
        i_prot.read_struct_begin()?;
        let mut f_1: Option<String> = None;
        let mut f_2: Option<ProbabilisticSamplingStrategy> = None;
        loop {
            let field_ident = i_prot.read_field_begin()?;
            if field_ident.field_type == TType::Stop {
                break;
            }
            let field_id = field_id(&field_ident)?;
            match field_id {
                1 => {
                    let val = i_prot.read_string()?;
                    f_1 = Some(val);
                }
                2 => {
                    let val = ProbabilisticSamplingStrategy::read_from_in_protocol(i_prot)?;
                    f_2 = Some(val);
                }
                _ => {
                    i_prot.skip(field_ident.field_type)?;
                }
            };
            i_prot.read_field_end()?;
        }
        i_prot.read_struct_end()?;
        verify_required_field_exists("OperationSamplingStrategy.operation", &f_1)?;
        verify_required_field_exists("OperationSamplingStrategy.probabilistic_sampling", &f_2)?;
        let ret = OperationSamplingStrategy {
            operation: f_1
                .expect("auto-generated code should have checked for presence of required fields"),
            probabilistic_sampling: f_2
                .expect("auto-generated code should have checked for presence of required fields"),
        };
        Ok(ret)
    }
    pub fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
        let struct_ident = TStructIdentifier::new("OperationSamplingStrategy");
        o_prot.write_struct_begin(&struct_ident)?;
        o_prot.write_field_begin(&TFieldIdentifier::new("operation", TType::String, 1))?;
        o_prot.write_string(&self.operation)?;
        o_prot.write_field_end()?;
        o_prot.write_field_begin(&TFieldIdentifier::new(
            "probabilisticSampling",
            TType::Struct,
            2,
        ))?;
        self.probabilistic_sampling.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
        o_prot.write_field_stop()?;
        o_prot.write_struct_end()
    }
}

//
// PerOperationSamplingStrategies
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PerOperationSamplingStrategies {
    pub default_sampling_probability: OrderedFloat<f64>,
    pub default_lower_bound_traces_per_second: OrderedFloat<f64>,
    pub per_operation_strategies: Vec<OperationSamplingStrategy>,
    pub default_upper_bound_traces_per_second: Option<OrderedFloat<f64>>,
}

impl PerOperationSamplingStrategies {
    pub fn new<F4>(
        default_sampling_probability: OrderedFloat<f64>,
        default_lower_bound_traces_per_second: OrderedFloat<f64>,
        per_operation_strategies: Vec<OperationSamplingStrategy>,
        default_upper_bound_traces_per_second: F4,
    ) -> PerOperationSamplingStrategies
    where
        F4: Into<Option<OrderedFloat<f64>>>,
    {
        PerOperationSamplingStrategies {
            default_sampling_probability,
            default_lower_bound_traces_per_second,
            per_operation_strategies,
            default_upper_bound_traces_per_second: default_upper_bound_traces_per_second.into(),
        }
    }
    pub fn read_from_in_protocol(
        i_prot: &mut dyn TInputProtocol,
    ) -> thrift::Result<PerOperationSamplingStrategies> {
        i_prot.read_struct_begin()?;
        let mut f_1: Option<OrderedFloat<f64>> = None;
        let mut f_2: Option<OrderedFloat<f64>> = None;
        let mut f_3: Option<Vec<OperationSamplingStrategy>> = None;
        let mut f_4: Option<OrderedFloat<f64>> = None;
        loop {
            let field_ident = i_prot.read_field_begin()?;
            if field_ident.field_type == TType::Stop {
                break;
            }
            let field_id = field_id(&field_ident)?;
            match field_id {
                1 => {
                    let val = OrderedFloat::from(i_prot.read_double()?);
                    f_1 = Some(val);
                }
                2 => {
                    let val = OrderedFloat::from(i_prot.read_double()?);
                    f_2 = Some(val);
                }
                3 => {
                    let list_ident = i_prot.read_list_begin()?;
                    let mut val: Vec<OperationSamplingStrategy> =
                        Vec::with_capacity(list_ident.size as usize);
                    for _ in 0..list_ident.size {
                        let list_elem_0 = OperationSamplingStrategy::read_from_in_protocol(i_prot)?;
                        val.push(list_elem_0);
                    }
                    i_prot.read_list_end()?;
                    f_3 = Some(val);
                }
                4 => {
                    let val = OrderedFloat::from(i_prot.read_double()?);
                    f_4 = Some(val);
                }
                _ => {
                    i_prot.skip(field_ident.field_type)?;
                }
            };
            i_prot.read_field_end()?;
        }
        i_prot.read_struct_end()?;
        verify_required_field_exists(
            "PerOperationSamplingStrategies.default_sampling_probability",
            &f_1,
        )?;
        verify_required_field_exists(
            "PerOperationSamplingStrategies.default_lower_bound_traces_per_second",
            &f_2,
        )?;
        verify_required_field_exists(
            "PerOperationSamplingStrategies.per_operation_strategies",
            &f_3,
        )?;
        let ret = PerOperationSamplingStrategies {
            default_sampling_probability: f_1
                .expect("auto-generated code should have checked for presence of required fields"),
            default_lower_bound_traces_per_second: f_2
                .expect("auto-generated code should have checked for presence of required fields"),
            per_operation_strategies: f_3
                .expect("auto-generated code should have checked for presence of required fields"),
            default_upper_bound_traces_per_second: f_4,
        };
        Ok(ret)
    }
    pub fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
        let struct_ident = TStructIdentifier::new("PerOperationSamplingStrategies");
        o_prot.write_struct_begin(&struct_ident)?;
        o_prot.write_field_begin(&TFieldIdentifier::new(
            "defaultSamplingProbability",
            TType::Double,
            1,
        ))?;
        o_prot.write_double(self.default_sampling_probability.into())?;
        o_prot.write_field_end()?;
        o_prot.write_field_begin(&TFieldIdentifier::new(
            "defaultLowerBoundTracesPerSecond",
            TType::Double,
            2,
        ))?;
        o_prot.write_double(self.default_lower_bound_traces_per_second.into())?;
        o_prot.write_field_end()?;
        o_prot.write_field_begin(&TFieldIdentifier::new(
            "perOperationStrategies",
            TType::List,
            3,
        ))?;
        o_prot.write_list_begin(&TListIdentifier::new(
            TType::Struct,
            self.per_operation_strategies.len() as i32,
        ))?;
        for e in &self.per_operation_strategies {
            e.write_to_out_protocol(o_prot)?;
            o_prot.write_list_end()?;
        }
        o_prot.write_field_end()?;
        if let Some(fld_var) = self.default_upper_bound_traces_per_second {
            o_prot.write_field_begin(&TFieldIdentifier::new(
                "defaultUpperBoundTracesPerSecond",
                TType::Double,
                4,
            ))?;
            o_prot.write_double(fld_var.into())?;
            o_prot.write_field_end()?;
        } else {
        }
        o_prot.write_field_stop()?;
        o_prot.write_struct_end()
    }
}

//
// SamplingStrategyResponse
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SamplingStrategyResponse {
    pub strategy_type: SamplingStrategyType,
    pub probabilistic_sampling: Option<ProbabilisticSamplingStrategy>,
    pub rate_limiting_sampling: Option<RateLimitingSamplingStrategy>,
    pub operation_sampling: Option<PerOperationSamplingStrategies>,
}

impl SamplingStrategyResponse {
    pub fn new<F2, F3, F4>(
        strategy_type: SamplingStrategyType,
        probabilistic_sampling: F2,
        rate_limiting_sampling: F3,
        operation_sampling: F4,
    ) -> SamplingStrategyResponse
    where
        F2: Into<Option<ProbabilisticSamplingStrategy>>,
        F3: Into<Option<RateLimitingSamplingStrategy>>,
        F4: Into<Option<PerOperationSamplingStrategies>>,
    {
        SamplingStrategyResponse {
            strategy_type,
            probabilistic_sampling: probabilistic_sampling.into(),
            rate_limiting_sampling: rate_limiting_sampling.into(),
            operation_sampling: operation_sampling.into(),
        }
    }
    pub fn read_from_in_protocol(
        i_prot: &mut dyn TInputProtocol,
    ) -> thrift::Result<SamplingStrategyResponse> {
        i_prot.read_struct_begin()?;
        let mut f_1: Option<SamplingStrategyType> = None;
        let mut f_2: Option<ProbabilisticSamplingStrategy> = None;
        let mut f_3: Option<RateLimitingSamplingStrategy> = None;
        let mut f_4: Option<PerOperationSamplingStrategies> = None;
        loop {
            let field_ident = i_prot.read_field_begin()?;
            if field_ident.field_type == TType::Stop {
                break;
            }
            let field_id = field_id(&field_ident)?;
            match field_id {
                1 => {
                    let val = SamplingStrategyType::read_from_in_protocol(i_prot)?;
                    f_1 = Some(val);
                }
                2 => {
                    let val = ProbabilisticSamplingStrategy::read_from_in_protocol(i_prot)?;
                    f_2 = Some(val);
                }
                3 => {
                    let val = RateLimitingSamplingStrategy::read_from_in_protocol(i_prot)?;
                    f_3 = Some(val);
                }
                4 => {
                    let val = PerOperationSamplingStrategies::read_from_in_protocol(i_prot)?;
                    f_4 = Some(val);
                }
                _ => {
                    i_prot.skip(field_ident.field_type)?;
                }
            };
            i_prot.read_field_end()?;
        }
        i_prot.read_struct_end()?;
        verify_required_field_exists("SamplingStrategyResponse.strategy_type", &f_1)?;
        let ret = SamplingStrategyResponse {
            strategy_type: f_1
                .expect("auto-generated code should have checked for presence of required fields"),
            probabilistic_sampling: f_2,
            rate_limiting_sampling: f_3,
            operation_sampling: f_4,
        };
        Ok(ret)
    }
    pub fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
        let struct_ident = TStructIdentifier::new("SamplingStrategyResponse");
        o_prot.write_struct_begin(&struct_ident)?;
        o_prot.write_field_begin(&TFieldIdentifier::new("strategyType", TType::I32, 1))?;
        self.strategy_type.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
        if let Some(ref fld_var) = self.probabilistic_sampling {
            o_prot.write_field_begin(&TFieldIdentifier::new(
                "probabilisticSampling",
                TType::Struct,
                2,
            ))?;
            fld_var.write_to_out_protocol(o_prot)?;
            o_prot.write_field_end()?;
        } else {
        }
        if let Some(ref fld_var) = self.rate_limiting_sampling {
            o_prot.write_field_begin(&TFieldIdentifier::new(
                "rateLimitingSampling",
                TType::Struct,
                3,
            ))?;
            fld_var.write_to_out_protocol(o_prot)?;
            o_prot.write_field_end()?;
        } else {
        }
        if let Some(ref fld_var) = self.operation_sampling {
            o_prot.write_field_begin(&TFieldIdentifier::new(
                "operationSampling",
                TType::Struct,
                4,
            ))?;
            fld_var.write_to_out_protocol(o_prot)?;
            o_prot.write_field_end()?;
        } else {
        }
        o_prot.write_field_stop()?;
        o_prot.write_struct_end()
    }
}

//
// SamplingManager service client
//

pub trait TSamplingManagerSyncClient {
    fn get_sampling_strategy(
        &mut self,
        service_name: String,
    ) -> thrift::Result<SamplingStrategyResponse>;
}

pub trait TSamplingManagerSyncClientMarker {}

pub struct SamplingManagerSyncClient<IP, OP>
where
    IP: TInputProtocol,
    OP: TOutputProtocol,
{
    _i_prot: IP,
    _o_prot: OP,
    _sequence_number: i32,
}

impl<IP, OP> SamplingManagerSyncClient<IP, OP>
where
    IP: TInputProtocol,
    OP: TOutputProtocol,
{
    pub fn new(input_protocol: IP, output_protocol: OP) -> SamplingManagerSyncClient<IP, OP> {
        SamplingManagerSyncClient {
            _i_prot: input_protocol,
            _o_prot: output_protocol,
            _sequence_number: 0,
        }
    }
}

impl<IP, OP> TThriftClient for SamplingManagerSyncClient<IP, OP>
where
    IP: TInputProtocol,
    OP: TOutputProtocol,
{
    fn i_prot_mut(&mut self) -> &mut dyn TInputProtocol {
        &mut self._i_prot
    }
    fn o_prot_mut(&mut self) -> &mut dyn TOutputProtocol {
        &mut self._o_prot
    }
    fn sequence_number(&self) -> i32 {
        self._sequence_number
    }
    fn increment_sequence_number(&mut self) -> i32 {
        self._sequence_number += 1;
        self._sequence_number
    }
}

impl<IP, OP> TSamplingManagerSyncClientMarker for SamplingManagerSyncClient<IP, OP>
where
    IP: TInputProtocol,
    OP: TOutputProtocol,
{
}

impl<C: TThriftClient + TSamplingManagerSyncClientMarker> TSamplingManagerSyncClient for C {
    fn get_sampling_strategy(
        &mut self,
        service_name: String,
    ) -> thrift::Result<SamplingStrategyResponse> {
        ({
            self.increment_sequence_number();
            let message_ident = TMessageIdentifier::new(
                "getSamplingStrategy",
                TMessageType::Call,
                self.sequence_number(),
            );
            let call_args = SamplingManagerGetSamplingStrategyArgs { service_name };
            self.o_prot_mut().write_message_begin(&message_ident)?;
            call_args.write_to_out_protocol(self.o_prot_mut())?;
            self.o_prot_mut().write_message_end()?;
            self.o_prot_mut().flush()
        })?;
        {
            let message_ident = self.i_prot_mut().read_message_begin()?;
            verify_expected_sequence_number(self.sequence_number(), message_ident.sequence_number)?;
            verify_expected_service_call("getSamplingStrategy", &message_ident.name)?;
            if message_ident.message_type == TMessageType::Exception {
                let remote_error =
                    thrift::Error::read_application_error_from_in_protocol(self.i_prot_mut())?;
                self.i_prot_mut().read_message_end()?;
                return Err(thrift::Error::Application(remote_error));
            }
            verify_expected_message_type(TMessageType::Reply, message_ident.message_type)?;
            let result =
                SamplingManagerGetSamplingStrategyResult::read_from_in_protocol(self.i_prot_mut())?;
            self.i_prot_mut().read_message_end()?;
            result.ok_or()
        }
    }
}

//
// SamplingManager service processor
//

pub trait SamplingManagerSyncHandler {
    fn handle_get_sampling_strategy(
        &self,
        service_name: String,
    ) -> thrift::Result<SamplingStrategyResponse>;
}

pub struct SamplingManagerSyncProcessor<H: SamplingManagerSyncHandler> {
    handler: H,
}

impl<H: SamplingManagerSyncHandler> SamplingManagerSyncProcessor<H> {
    pub fn new(handler: H) -> SamplingManagerSyncProcessor<H> {
        SamplingManagerSyncProcessor { handler }
    }
    fn process_get_sampling_strategy(
        &self,
        incoming_sequence_number: i32,
        i_prot: &mut dyn TInputProtocol,
        o_prot: &mut dyn TOutputProtocol,
    ) -> thrift::Result<()> {
        TSamplingManagerProcessFunctions::process_get_sampling_strategy(
            &self.handler,
            incoming_sequence_number,
            i_prot,
            o_prot,
        )
    }
}

pub struct TSamplingManagerProcessFunctions;

impl TSamplingManagerProcessFunctions {
    pub fn process_get_sampling_strategy<H: SamplingManagerSyncHandler>(
        handler: &H,
        incoming_sequence_number: i32,
        i_prot: &mut dyn TInputProtocol,
        o_prot: &mut dyn TOutputProtocol,
    ) -> thrift::Result<()> {
        let args = SamplingManagerGetSamplingStrategyArgs::read_from_in_protocol(i_prot)?;
        match handler.handle_get_sampling_strategy(args.service_name) {
            Ok(handler_return) => {
                let message_ident = TMessageIdentifier::new(
                    "getSamplingStrategy",
                    TMessageType::Reply,
                    incoming_sequence_number,
                );
                o_prot.write_message_begin(&message_ident)?;
                let ret = SamplingManagerGetSamplingStrategyResult {
                    result_value: Some(handler_return),
                };
                ret.write_to_out_protocol(o_prot)?;
                o_prot.write_message_end()?;
                o_prot.flush()
            }
            Err(e) => match e {
                thrift::Error::Application(app_err) => {
                    let message_ident = TMessageIdentifier::new(
                        "getSamplingStrategy",
                        TMessageType::Exception,
                        incoming_sequence_number,
                    );
                    o_prot.write_message_begin(&message_ident)?;
                    thrift::Error::write_application_error_to_out_protocol(&app_err, o_prot)?;
                    o_prot.write_message_end()?;
                    o_prot.flush()
                }
                _ => {
                    let ret_err =
                        { ApplicationError::new(ApplicationErrorKind::Unknown, e.description()) };
                    let message_ident = TMessageIdentifier::new(
                        "getSamplingStrategy",
                        TMessageType::Exception,
                        incoming_sequence_number,
                    );
                    o_prot.write_message_begin(&message_ident)?;
                    thrift::Error::write_application_error_to_out_protocol(&ret_err, o_prot)?;
                    o_prot.write_message_end()?;
                    o_prot.flush()
                }
            },
        }
    }
}

impl<H: SamplingManagerSyncHandler> TProcessor for SamplingManagerSyncProcessor<H> {
    fn process(
        &self,
        i_prot: &mut dyn TInputProtocol,
        o_prot: &mut dyn TOutputProtocol,
    ) -> thrift::Result<()> {
        let message_ident = i_prot.read_message_begin()?;
        let res = match &*message_ident.name {
            "getSamplingStrategy" => {
                self.process_get_sampling_strategy(message_ident.sequence_number, i_prot, o_prot)
            }
            method => Err(thrift::Error::Application(ApplicationError::new(
                ApplicationErrorKind::UnknownMethod,
                format!("unknown method {}", method),
            ))),
        };
        thrift::server::handle_process_result(&message_ident, res, o_prot)
    }
}

//
// SamplingManagerGetSamplingStrategyArgs
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct SamplingManagerGetSamplingStrategyArgs {
    service_name: String,
}

impl SamplingManagerGetSamplingStrategyArgs {
    fn read_from_in_protocol(
        i_prot: &mut dyn TInputProtocol,
    ) -> thrift::Result<SamplingManagerGetSamplingStrategyArgs> {
        i_prot.read_struct_begin()?;
        let mut f_1: Option<String> = None;
        loop {
            let field_ident = i_prot.read_field_begin()?;
            if field_ident.field_type == TType::Stop {
                break;
            }
            let field_id = field_id(&field_ident)?;
            match field_id {
                1 => {
                    let val = i_prot.read_string()?;
                    f_1 = Some(val);
                }
                _ => {
                    i_prot.skip(field_ident.field_type)?;
                }
            };
            i_prot.read_field_end()?;
        }
        i_prot.read_struct_end()?;
        verify_required_field_exists("SamplingManagerGetSamplingStrategyArgs.service_name", &f_1)?;
        let ret = SamplingManagerGetSamplingStrategyArgs {
            service_name: f_1
                .expect("auto-generated code should have checked for presence of required fields"),
        };
        Ok(ret)
    }
    fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
        let struct_ident = TStructIdentifier::new("getSamplingStrategy_args");
        o_prot.write_struct_begin(&struct_ident)?;
        o_prot.write_field_begin(&TFieldIdentifier::new("serviceName", TType::String, 1))?;
        o_prot.write_string(&self.service_name)?;
        o_prot.write_field_end()?;
        o_prot.write_field_stop()?;
        o_prot.write_struct_end()
    }
}

//
// SamplingManagerGetSamplingStrategyResult
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct SamplingManagerGetSamplingStrategyResult {
    result_value: Option<SamplingStrategyResponse>,
}

impl SamplingManagerGetSamplingStrategyResult {
    fn read_from_in_protocol(
        i_prot: &mut dyn TInputProtocol,
    ) -> thrift::Result<SamplingManagerGetSamplingStrategyResult> {
        i_prot.read_struct_begin()?;
        let mut f_0: Option<SamplingStrategyResponse> = None;
        loop {
            let field_ident = i_prot.read_field_begin()?;
            if field_ident.field_type == TType::Stop {
                break;
            }
            let field_id = field_id(&field_ident)?;
            match field_id {
                0 => {
                    let val = SamplingStrategyResponse::read_from_in_protocol(i_prot)?;
                    f_0 = Some(val);
                }
                _ => {
                    i_prot.skip(field_ident.field_type)?;
                }
            };
            i_prot.read_field_end()?;
        }
        i_prot.read_struct_end()?;
        let ret = SamplingManagerGetSamplingStrategyResult { result_value: f_0 };
        Ok(ret)
    }
    fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
        let struct_ident = TStructIdentifier::new("SamplingManagerGetSamplingStrategyResult");
        o_prot.write_struct_begin(&struct_ident)?;
        if let Some(ref fld_var) = self.result_value {
            o_prot.write_field_begin(&TFieldIdentifier::new("result_value", TType::Struct, 0))?;
            fld_var.write_to_out_protocol(o_prot)?;
            o_prot.write_field_end()?;
        } else {
        }
        o_prot.write_field_stop()?;
        o_prot.write_struct_end()
    }
    fn ok_or(self) -> thrift::Result<SamplingStrategyResponse> {
        if self.result_value.is_some() {
            Ok(self.result_value.unwrap())
        } else {
            Err(thrift::Error::Application(ApplicationError::new(
                ApplicationErrorKind::MissingResult,
                "no result received for SamplingManagerGetSamplingStrategy",
            )))
        }
    }
}
