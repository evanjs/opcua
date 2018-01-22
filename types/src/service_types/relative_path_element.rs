// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use string::*;
#[allow(unused_imports)]
use byte_string::ByteString;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::enums::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use service_types::impls::*;
#[allow(unused_imports)]
use node_ids::ObjectId;
#[allow(unused_imports)]
use status_codes::StatusCode;

/// An element in a relative path.
#[derive(Debug, Clone, PartialEq)]
pub struct RelativePathElement {
    pub reference_type_id: NodeId,
    pub is_inverse: Boolean,
    pub include_subtypes: Boolean,
    pub target_name: QualifiedName,
}

impl MessageInfo for RelativePathElement {
    fn object_id(&self) -> ObjectId {
        ObjectId::RelativePathElement_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RelativePathElement> for RelativePathElement {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.reference_type_id.byte_len();
        size += self.is_inverse.byte_len();
        size += self.include_subtypes.byte_len();
        size += self.target_name.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_inverse.encode(stream)?;
        size += self.include_subtypes.encode(stream)?;
        size += self.target_name.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let reference_type_id = NodeId::decode(stream)?;
        let is_inverse = Boolean::decode(stream)?;
        let include_subtypes = Boolean::decode(stream)?;
        let target_name = QualifiedName::decode(stream)?;
        Ok(RelativePathElement {
            reference_type_id,
            is_inverse,
            include_subtypes,
            target_name,
        })
    }
}