#[derive(Debug)]
pub struct PDF {
    pub version: String,
    pub objects: Vec<Object>,
}
// impl Eq for PDF {}
#[derive(Debug, PartialEq)]
pub struct Object {
    pub number: i64,
    pub gen: i64,
    pub data: AnyPDFData,
}
#[derive(Debug, PartialEq)]
pub enum AnyPDFData {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Name(String),
    String(String),
    Array(Vec<AnyPDFData>),
    Dictionary(Vec<(String, AnyPDFData)>),
    Stream(Vec<(String, AnyPDFData)>, Vec<u8>),
    ObjRef(i64, i64),
}
impl Clone for AnyPDFData {
    fn clone(&self) -> Self {
        match self {
            AnyPDFData::Boolean(b) => AnyPDFData::Boolean(*b),
            AnyPDFData::Integer(i) => AnyPDFData::Integer(*i),
            AnyPDFData::Real(r) => AnyPDFData::Real(*r),
            AnyPDFData::Name(s) => AnyPDFData::Name(s.clone()),
            AnyPDFData::String(s) => AnyPDFData::String(s.clone()),
            AnyPDFData::Array(a) => AnyPDFData::Array(a.clone()),
            AnyPDFData::Dictionary(d) => AnyPDFData::Dictionary(d.clone()),
            AnyPDFData::Stream(a, b) => AnyPDFData::Stream(a.clone(), b.clone()),
            AnyPDFData::ObjRef(n, g) => AnyPDFData::ObjRef(*n, *g),
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object {
            number: self.number,
            gen: self.gen,
            data: self.data.clone(),
        }
    }
}

impl Clone for PDF {
    fn clone(&self) -> Self {
        PDF {
            version: self.version.clone(),
            objects: self.objects.clone(),
        }
    }
}
