use crate::*;

use heapless::{ArrayLength, String};
/// Base enum for all MQTT packet types.
///
/// This is the main type you'll be interacting with, as an output of [`decode()`] and an input of
/// [`encode()`]. Most variants can be constructed directly without using methods.
///
/// ```
/// # use mqttrs::*;
/// # use core::convert::TryFrom;
/// // Simplest form
/// let pkt = Packet::Connack(Connack { session_present: false,
///                                     code: ConnectReturnCode::Accepted });
/// // Using `Into` trait
/// let publish = Publish { dup: false,
///                         qospid: QosPid::AtMostOnce,
///                         retain: false,
///                         topic_name: "to/pic".into(),
///                         payload: "payload".into() };
/// let pkt: Packet = publish.into();
/// // Identifyer-only packets
/// let pkt = Packet::Puback(Pid::try_from(42).unwrap());
/// ```
///
/// [`encode()`]: fn.encode.html
/// [`decode()`]: fn.decode.html
#[derive(Debug, Clone, PartialEq)]
pub enum Packet<
    ClientIdLen,
    UsernameLen,
    PasswordLen,
    SubReq,
    UnsubReq,
    TopicLen,
    PayloadLen,
    SubackReq,
> where
    ClientIdLen: ArrayLength<u8>,
    UsernameLen: ArrayLength<u8>,
    PasswordLen: ArrayLength<u8>,
    TopicLen: ArrayLength<u8>,
    SubReq: ArrayLength<SubscribeTopic<TopicLen>>,
    SubackReq: ArrayLength<SubscribeReturnCodes>,
    UnsubReq: ArrayLength<String<TopicLen>>,
    PayloadLen: ArrayLength<u8>,
{
    /// [MQTT 3.1](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718028)
    Connect(Connect<ClientIdLen, UsernameLen, PasswordLen, TopicLen, PayloadLen>),
    /// [MQTT 3.2](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718033)
    Connack(Connack),
    /// [MQTT 3.3](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718037)
    Publish(Publish<TopicLen, PayloadLen>),
    /// [MQTT 3.4](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718043)
    Puback(Pid),
    /// [MQTT 3.5](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718048)
    Pubrec(Pid),
    /// [MQTT 3.6](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718053)
    Pubrel(Pid),
    /// [MQTT 3.7](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718058)
    Pubcomp(Pid),
    /// [MQTT 3.8](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718063)
    Subscribe(Subscribe<SubReq, TopicLen>),
    /// [MQTT 3.9](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718068)
    Suback(Suback<SubackReq>),
    /// [MQTT 3.10](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718072)
    Unsubscribe(Unsubscribe<UnsubReq, TopicLen>),
    /// [MQTT 3.11](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718077)
    Unsuback(Pid),
    /// [MQTT 3.12](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718081)
    Pingreq,
    /// [MQTT 3.13](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718086)
    Pingresp,
    /// [MQTT 3.14](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718090)
    Disconnect,
}
impl<ClientIdLen, UsernameLen, PasswordLen, SubReq, UnsubReq, TopicLen, PayloadLen, SubackReq>
    Packet<ClientIdLen, UsernameLen, PasswordLen, SubReq, UnsubReq, TopicLen, PayloadLen, SubackReq>
where
    ClientIdLen: ArrayLength<u8>,
    UsernameLen: ArrayLength<u8>,
    PasswordLen: ArrayLength<u8>,
    TopicLen: ArrayLength<u8>,
    SubReq: ArrayLength<SubscribeTopic<TopicLen>>,
    SubackReq: ArrayLength<SubscribeReturnCodes>,
    UnsubReq: ArrayLength<String<TopicLen>>,
    PayloadLen: ArrayLength<u8>,
{
    /// Return the packet type variant.
    ///
    /// This can be used for matching, categorising, debuging, etc. Most users will match directly
    /// on `Packet` instead.
    pub fn get_type(&self) -> PacketType {
        match self {
            Packet::Connect(_) => PacketType::Connect,
            Packet::Connack(_) => PacketType::Connack,
            Packet::Publish(_) => PacketType::Publish,
            Packet::Puback(_) => PacketType::Puback,
            Packet::Pubrec(_) => PacketType::Pubrec,
            Packet::Pubrel(_) => PacketType::Pubrel,
            Packet::Pubcomp(_) => PacketType::Pubcomp,
            Packet::Subscribe(_) => PacketType::Subscribe,
            Packet::Suback(_) => PacketType::Suback,
            Packet::Unsubscribe(_) => PacketType::Unsubscribe,
            Packet::Unsuback(_) => PacketType::Unsuback,
            Packet::Pingreq => PacketType::Pingreq,
            Packet::Pingresp => PacketType::Pingresp,
            Packet::Disconnect => PacketType::Disconnect,
        }
    }
}

impl<ClientIdLen, UsernameLen, PasswordLen, SubReq, UnsubReq, TopicLen, PayloadLen, SubackReq>
    From<Connack>
    for Packet<
        ClientIdLen,
        UsernameLen,
        PasswordLen,
        SubReq,
        UnsubReq,
        TopicLen,
        PayloadLen,
        SubackReq,
    >
where
    ClientIdLen: ArrayLength<u8>,
    UsernameLen: ArrayLength<u8>,
    PasswordLen: ArrayLength<u8>,
    TopicLen: ArrayLength<u8>,
    SubReq: ArrayLength<SubscribeTopic<TopicLen>>,
    SubackReq: ArrayLength<SubscribeReturnCodes>,
    UnsubReq: ArrayLength<String<TopicLen>>,
    PayloadLen: ArrayLength<u8>,
{
    fn from(p: Connack) -> Self {
        Packet::Connack(p)
    }
}

impl<ClientIdLen, UsernameLen, PasswordLen, SubReq, UnsubReq, TopicLen, PayloadLen, SubackReq>
    From<Connect<ClientIdLen, UsernameLen, PasswordLen, TopicLen, PayloadLen>>
    for Packet<
        ClientIdLen,
        UsernameLen,
        PasswordLen,
        SubReq,
        UnsubReq,
        TopicLen,
        PayloadLen,
        SubackReq,
    >
where
    ClientIdLen: ArrayLength<u8>,
    UsernameLen: ArrayLength<u8>,
    PasswordLen: ArrayLength<u8>,
    TopicLen: ArrayLength<u8>,
    SubReq: ArrayLength<SubscribeTopic<TopicLen>>,
    SubackReq: ArrayLength<SubscribeReturnCodes>,
    UnsubReq: ArrayLength<String<TopicLen>>,
    PayloadLen: ArrayLength<u8>,
{
    fn from(p: Connect<ClientIdLen, UsernameLen, PasswordLen, TopicLen, PayloadLen>) -> Self {
        Packet::Connect(p)
    }
}

impl<ClientIdLen, UsernameLen, PasswordLen, SubReq, UnsubReq, TopicLen, PayloadLen, SubackReq>
    From<Publish<TopicLen, PayloadLen>>
    for Packet<
        ClientIdLen,
        UsernameLen,
        PasswordLen,
        SubReq,
        UnsubReq,
        TopicLen,
        PayloadLen,
        SubackReq,
    >
where
    ClientIdLen: ArrayLength<u8>,
    UsernameLen: ArrayLength<u8>,
    PasswordLen: ArrayLength<u8>,
    TopicLen: ArrayLength<u8>,
    SubReq: ArrayLength<SubscribeTopic<TopicLen>>,
    SubackReq: ArrayLength<SubscribeReturnCodes>,
    UnsubReq: ArrayLength<String<TopicLen>>,
    PayloadLen: ArrayLength<u8>,
{
    fn from(p: Publish<TopicLen, PayloadLen>) -> Self {
        Packet::Publish(p)
    }
}

impl<ClientIdLen, UsernameLen, PasswordLen, SubReq, UnsubReq, TopicLen, PayloadLen, SubackReq>
    From<Subscribe<SubReq, TopicLen>>
    for Packet<
        ClientIdLen,
        UsernameLen,
        PasswordLen,
        SubReq,
        UnsubReq,
        TopicLen,
        PayloadLen,
        SubackReq,
    >
where
    ClientIdLen: ArrayLength<u8>,
    UsernameLen: ArrayLength<u8>,
    PasswordLen: ArrayLength<u8>,
    TopicLen: ArrayLength<u8>,
    SubReq: ArrayLength<SubscribeTopic<TopicLen>>,
    SubackReq: ArrayLength<SubscribeReturnCodes>,
    UnsubReq: ArrayLength<String<TopicLen>>,
    PayloadLen: ArrayLength<u8>,
{
    fn from(p: Subscribe<SubReq, TopicLen>) -> Self {
        Packet::Subscribe(p)
    }
}

impl<ClientIdLen, UsernameLen, PasswordLen, SubReq, UnsubReq, TopicLen, PayloadLen, SubackReq>
    From<Suback<SubackReq>>
    for Packet<
        ClientIdLen,
        UsernameLen,
        PasswordLen,
        SubReq,
        UnsubReq,
        TopicLen,
        PayloadLen,
        SubackReq,
    >
where
    ClientIdLen: ArrayLength<u8>,
    UsernameLen: ArrayLength<u8>,
    PasswordLen: ArrayLength<u8>,
    TopicLen: ArrayLength<u8>,
    SubReq: ArrayLength<SubscribeTopic<TopicLen>>,
    SubackReq: ArrayLength<SubscribeReturnCodes>,
    UnsubReq: ArrayLength<String<TopicLen>>,
    PayloadLen: ArrayLength<u8>,
{
    fn from(p: Suback<SubackReq>) -> Self {
        Packet::Suback(p)
    }
}

impl<ClientIdLen, UsernameLen, PasswordLen, SubReq, UnsubReq, TopicLen, PayloadLen, SubackReq>
    From<Unsubscribe<UnsubReq, TopicLen>>
    for Packet<
        ClientIdLen,
        UsernameLen,
        PasswordLen,
        SubReq,
        UnsubReq,
        TopicLen,
        PayloadLen,
        SubackReq,
    >
where
    ClientIdLen: ArrayLength<u8>,
    UsernameLen: ArrayLength<u8>,
    PasswordLen: ArrayLength<u8>,
    TopicLen: ArrayLength<u8>,
    SubReq: ArrayLength<SubscribeTopic<TopicLen>>,
    SubackReq: ArrayLength<SubscribeReturnCodes>,
    UnsubReq: ArrayLength<String<TopicLen>>,
    PayloadLen: ArrayLength<u8>,
{
    fn from(p: Unsubscribe<UnsubReq, TopicLen>) -> Self {
        Packet::Unsubscribe(p)
    }
}

/// Packet type variant, without the associated data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PacketType {
    Connect,
    Connack,
    Publish,
    Puback,
    Pubrec,
    Pubrel,
    Pubcomp,
    Subscribe,
    Suback,
    Unsubscribe,
    Unsuback,
    Pingreq,
    Pingresp,
    Disconnect,
}
