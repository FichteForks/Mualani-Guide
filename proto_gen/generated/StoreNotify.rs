// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by protoc 29.0-rc1
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `StoreNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:Mualani.Guide.Weapon)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Weapon {
    // message fields
    // @@protoc_insertion_point(field:Mualani.Guide.Weapon.level)
    pub level: u32,
    // @@protoc_insertion_point(field:Mualani.Guide.Weapon.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:Mualani.Guide.Weapon.upgrade_level)
    pub upgrade_level: u32,
    // @@protoc_insertion_point(field:Mualani.Guide.Weapon.affix_map)
    pub affix_map: ::std::collections::HashMap<u32, u32>,
    // special fields
    // @@protoc_insertion_point(special_field:Mualani.Guide.Weapon.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Weapon {
    fn default() -> &'a Weapon {
        <Weapon as ::protobuf::Message>::default_instance()
    }
}

impl Weapon {
    pub fn new() -> Weapon {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &Weapon| { &m.level },
            |m: &mut Weapon| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &Weapon| { &m.exp },
            |m: &mut Weapon| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "upgrade_level",
            |m: &Weapon| { &m.upgrade_level },
            |m: &mut Weapon| { &mut m.upgrade_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "affix_map",
            |m: &Weapon| { &m.affix_map },
            |m: &mut Weapon| { &mut m.affix_map },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Weapon>(
            "Weapon",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Weapon {
    const NAME: &'static str = "Weapon";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.level = is.read_uint32()?;
                },
                16 => {
                    self.exp = is.read_uint32()?;
                },
                24 => {
                    self.upgrade_level = is.read_uint32()?;
                },
                34 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.affix_map.insert(key, value);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.level);
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.exp);
        }
        if self.upgrade_level != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.upgrade_level);
        }
        for (k, v) in &self.affix_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != 0 {
            os.write_uint32(1, self.level)?;
        }
        if self.exp != 0 {
            os.write_uint32(2, self.exp)?;
        }
        if self.upgrade_level != 0 {
            os.write_uint32(3, self.upgrade_level)?;
        }
        for (k, v) in &self.affix_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(34)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Weapon {
        Weapon::new()
    }

    fn clear(&mut self) {
        self.level = 0;
        self.exp = 0;
        self.upgrade_level = 0;
        self.affix_map.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Weapon {
        static instance: ::protobuf::rt::Lazy<Weapon> = ::protobuf::rt::Lazy::new();
        instance.get(Weapon::new)
    }
}

impl ::protobuf::MessageFull for Weapon {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Weapon").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Weapon {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:Mualani.Guide.Reliq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Reliq {
    // message fields
    // @@protoc_insertion_point(field:Mualani.Guide.Reliq.level)
    pub level: u32,
    // @@protoc_insertion_point(field:Mualani.Guide.Reliq.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:Mualani.Guide.Reliq.upgrade_level)
    pub upgrade_level: u32,
    // @@protoc_insertion_point(field:Mualani.Guide.Reliq.main_prop)
    pub main_prop: u32,
    // @@protoc_insertion_point(field:Mualani.Guide.Reliq.append_prop_id_list)
    pub append_prop_id_list: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:Mualani.Guide.Reliq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Reliq {
    fn default() -> &'a Reliq {
        <Reliq as ::protobuf::Message>::default_instance()
    }
}

impl Reliq {
    pub fn new() -> Reliq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &Reliq| { &m.level },
            |m: &mut Reliq| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &Reliq| { &m.exp },
            |m: &mut Reliq| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "upgrade_level",
            |m: &Reliq| { &m.upgrade_level },
            |m: &mut Reliq| { &mut m.upgrade_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "main_prop",
            |m: &Reliq| { &m.main_prop },
            |m: &mut Reliq| { &mut m.main_prop },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "append_prop_id_list",
            |m: &Reliq| { &m.append_prop_id_list },
            |m: &mut Reliq| { &mut m.append_prop_id_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Reliq>(
            "Reliq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Reliq {
    const NAME: &'static str = "Reliq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.level = is.read_uint32()?;
                },
                16 => {
                    self.exp = is.read_uint32()?;
                },
                24 => {
                    self.upgrade_level = is.read_uint32()?;
                },
                32 => {
                    self.main_prop = is.read_uint32()?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.append_prop_id_list)?;
                },
                40 => {
                    self.append_prop_id_list.push(is.read_uint32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.level);
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.exp);
        }
        if self.upgrade_level != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.upgrade_level);
        }
        if self.main_prop != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.main_prop);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(5, &self.append_prop_id_list);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != 0 {
            os.write_uint32(1, self.level)?;
        }
        if self.exp != 0 {
            os.write_uint32(2, self.exp)?;
        }
        if self.upgrade_level != 0 {
            os.write_uint32(3, self.upgrade_level)?;
        }
        if self.main_prop != 0 {
            os.write_uint32(4, self.main_prop)?;
        }
        os.write_repeated_packed_uint32(5, &self.append_prop_id_list)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Reliq {
        Reliq::new()
    }

    fn clear(&mut self) {
        self.level = 0;
        self.exp = 0;
        self.upgrade_level = 0;
        self.main_prop = 0;
        self.append_prop_id_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Reliq {
        static instance: Reliq = Reliq {
            level: 0,
            exp: 0,
            upgrade_level: 0,
            main_prop: 0,
            append_prop_id_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Reliq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Reliq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Reliq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Reliq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:Mualani.Guide.Equipment)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Equipment {
    // message fields
    // @@protoc_insertion_point(field:Mualani.Guide.Equipment.locked)
    pub locked: bool,
    // @@protoc_insertion_point(field:Mualani.Guide.Equipment.reliq)
    pub reliq: ::protobuf::MessageField<Reliq>,
    // @@protoc_insertion_point(field:Mualani.Guide.Equipment.weapon)
    pub weapon: ::protobuf::MessageField<Weapon>,
    // special fields
    // @@protoc_insertion_point(special_field:Mualani.Guide.Equipment.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Equipment {
    fn default() -> &'a Equipment {
        <Equipment as ::protobuf::Message>::default_instance()
    }
}

impl Equipment {
    pub fn new() -> Equipment {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "locked",
            |m: &Equipment| { &m.locked },
            |m: &mut Equipment| { &mut m.locked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Reliq>(
            "reliq",
            |m: &Equipment| { &m.reliq },
            |m: &mut Equipment| { &mut m.reliq },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Weapon>(
            "weapon",
            |m: &Equipment| { &m.weapon },
            |m: &mut Equipment| { &mut m.weapon },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Equipment>(
            "Equipment",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Equipment {
    const NAME: &'static str = "Equipment";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.locked = is.read_bool()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.reliq)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.weapon)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.locked != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.reliq.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.weapon.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.locked != false {
            os.write_bool(3, self.locked)?;
        }
        if let Some(v) = self.reliq.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.weapon.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Equipment {
        Equipment::new()
    }

    fn clear(&mut self) {
        self.locked = false;
        self.reliq.clear();
        self.weapon.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Equipment {
        static instance: Equipment = Equipment {
            locked: false,
            reliq: ::protobuf::MessageField::none(),
            weapon: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Equipment {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Equipment").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Equipment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Equipment {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:Mualani.Guide.Material)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Material {
    // message fields
    // @@protoc_insertion_point(field:Mualani.Guide.Material.count)
    pub count: u64,
    // special fields
    // @@protoc_insertion_point(special_field:Mualani.Guide.Material.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Material {
    fn default() -> &'a Material {
        <Material as ::protobuf::Message>::default_instance()
    }
}

impl Material {
    pub fn new() -> Material {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count",
            |m: &Material| { &m.count },
            |m: &mut Material| { &mut m.count },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Material>(
            "Material",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Material {
    const NAME: &'static str = "Material";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.count = is.read_uint64()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.count != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.count);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.count != 0 {
            os.write_uint64(1, self.count)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Material {
        Material::new()
    }

    fn clear(&mut self) {
        self.count = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Material {
        static instance: Material = Material {
            count: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Material {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Material").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Material {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Material {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:Mualani.Guide.ItemInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ItemInfo {
    // message fields
    // @@protoc_insertion_point(field:Mualani.Guide.ItemInfo.id)
    pub id: u64,
    // @@protoc_insertion_point(field:Mualani.Guide.ItemInfo.guid)
    pub guid: u64,
    // @@protoc_insertion_point(field:Mualani.Guide.ItemInfo.material)
    pub material: ::protobuf::MessageField<Material>,
    // @@protoc_insertion_point(field:Mualani.Guide.ItemInfo.equip)
    pub equip: ::protobuf::MessageField<Equipment>,
    // special fields
    // @@protoc_insertion_point(special_field:Mualani.Guide.ItemInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ItemInfo {
    fn default() -> &'a ItemInfo {
        <ItemInfo as ::protobuf::Message>::default_instance()
    }
}

impl ItemInfo {
    pub fn new() -> ItemInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &ItemInfo| { &m.id },
            |m: &mut ItemInfo| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "guid",
            |m: &ItemInfo| { &m.guid },
            |m: &mut ItemInfo| { &mut m.guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Material>(
            "material",
            |m: &ItemInfo| { &m.material },
            |m: &mut ItemInfo| { &mut m.material },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Equipment>(
            "equip",
            |m: &ItemInfo| { &m.equip },
            |m: &mut ItemInfo| { &mut m.equip },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ItemInfo>(
            "ItemInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ItemInfo {
    const NAME: &'static str = "ItemInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = is.read_uint64()?;
                },
                16 => {
                    self.guid = is.read_uint64()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.material)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.equip)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.id);
        }
        if self.guid != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.guid);
        }
        if let Some(v) = self.material.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.equip.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_uint64(1, self.id)?;
        }
        if self.guid != 0 {
            os.write_uint64(2, self.guid)?;
        }
        if let Some(v) = self.material.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.equip.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ItemInfo {
        ItemInfo::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.guid = 0;
        self.material.clear();
        self.equip.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ItemInfo {
        static instance: ItemInfo = ItemInfo {
            id: 0,
            guid: 0,
            material: ::protobuf::MessageField::none(),
            equip: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ItemInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ItemInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ItemInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ItemInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:Mualani.Guide.StoreNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StoreNotify {
    // message fields
    // @@protoc_insertion_point(field:Mualani.Guide.StoreNotify.Limit)
    pub Limit: u64,
    // @@protoc_insertion_point(field:Mualani.Guide.StoreNotify.StoreType)
    pub StoreType: u64,
    // @@protoc_insertion_point(field:Mualani.Guide.StoreNotify.items)
    pub items: ::std::vec::Vec<ItemInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:Mualani.Guide.StoreNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StoreNotify {
    fn default() -> &'a StoreNotify {
        <StoreNotify as ::protobuf::Message>::default_instance()
    }
}

impl StoreNotify {
    pub fn new() -> StoreNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Limit",
            |m: &StoreNotify| { &m.Limit },
            |m: &mut StoreNotify| { &mut m.Limit },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "StoreType",
            |m: &StoreNotify| { &m.StoreType },
            |m: &mut StoreNotify| { &mut m.StoreType },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "items",
            |m: &StoreNotify| { &m.items },
            |m: &mut StoreNotify| { &mut m.items },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StoreNotify>(
            "StoreNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StoreNotify {
    const NAME: &'static str = "StoreNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.Limit = is.read_uint64()?;
                },
                96 => {
                    self.StoreType = is.read_uint64()?;
                },
                114 => {
                    self.items.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.Limit != 0 {
            my_size += ::protobuf::rt::uint64_size(4, self.Limit);
        }
        if self.StoreType != 0 {
            my_size += ::protobuf::rt::uint64_size(12, self.StoreType);
        }
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.Limit != 0 {
            os.write_uint64(4, self.Limit)?;
        }
        if self.StoreType != 0 {
            os.write_uint64(12, self.StoreType)?;
        }
        for v in &self.items {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StoreNotify {
        StoreNotify::new()
    }

    fn clear(&mut self) {
        self.Limit = 0;
        self.StoreType = 0;
        self.items.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StoreNotify {
        static instance: StoreNotify = StoreNotify {
            Limit: 0,
            StoreType: 0,
            items: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StoreNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StoreNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StoreNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StoreNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11StoreNotify.proto\x12\rMualani.Guide\"\xd4\x01\n\x06Weapon\x12\x14\
    \n\x05level\x18\x01\x20\x01(\rR\x05level\x12\x10\n\x03exp\x18\x02\x20\
    \x01(\rR\x03exp\x12#\n\rupgrade_level\x18\x03\x20\x01(\rR\x0cupgradeLeve\
    l\x12@\n\taffix_map\x18\x04\x20\x03(\x0b2#.Mualani.Guide.Weapon.AffixMap\
    EntryR\x08affixMap\x1a;\n\rAffixMapEntry\x12\x10\n\x03key\x18\x01\x20\
    \x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\
    \x01\"\xa0\x01\n\x05Reliq\x12\x14\n\x05level\x18\x01\x20\x01(\rR\x05leve\
    l\x12\x10\n\x03exp\x18\x02\x20\x01(\rR\x03exp\x12#\n\rupgrade_level\x18\
    \x03\x20\x01(\rR\x0cupgradeLevel\x12\x1b\n\tmain_prop\x18\x04\x20\x01(\r\
    R\x08mainProp\x12-\n\x13append_prop_id_list\x18\x05\x20\x03(\rR\x10appen\
    dPropIdList\"~\n\tEquipment\x12\x16\n\x06locked\x18\x03\x20\x01(\x08R\
    \x06locked\x12*\n\x05reliq\x18\x01\x20\x01(\x0b2\x14.Mualani.Guide.Reliq\
    R\x05reliq\x12-\n\x06weapon\x18\x02\x20\x01(\x0b2\x15.Mualani.Guide.Weap\
    onR\x06weapon\"\x20\n\x08Material\x12\x14\n\x05count\x18\x01\x20\x01(\
    \x04R\x05count\"\xb4\x01\n\x08ItemInfo\x12\x0e\n\x02id\x18\x01\x20\x01(\
    \x04R\x02id\x12\x12\n\x04guid\x18\x02\x20\x01(\x04R\x04guid\x128\n\x08ma\
    terial\x18\x05\x20\x01(\x0b2\x17.Mualani.Guide.MaterialH\0R\x08material\
    \x88\x01\x01\x123\n\x05equip\x18\x06\x20\x01(\x0b2\x18.Mualani.Guide.Equ\
    ipmentH\x01R\x05equip\x88\x01\x01B\x0b\n\t_materialB\x08\n\x06_equip\"p\
    \n\x0bStoreNotify\x12\x14\n\x05Limit\x18\x04\x20\x01(\x04R\x05Limit\x12\
    \x1c\n\tStoreType\x18\x0c\x20\x01(\x04R\tStoreType\x12-\n\x05items\x18\
    \x0e\x20\x03(\x0b2\x17.Mualani.Guide.ItemInfoR\x05itemsb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(6);
            messages.push(Weapon::generated_message_descriptor_data());
            messages.push(Reliq::generated_message_descriptor_data());
            messages.push(Equipment::generated_message_descriptor_data());
            messages.push(Material::generated_message_descriptor_data());
            messages.push(ItemInfo::generated_message_descriptor_data());
            messages.push(StoreNotify::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
