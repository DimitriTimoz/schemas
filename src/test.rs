/// The header section of the page.
#[derive()]
pub struct WPHeader {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WPHeader {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A doctor's office.
#[derive()]
pub struct Physician {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Physician {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A music recording (track), usually a single song.
#[derive()]
pub struct MusicRecording {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicRecording {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A chemical substance is 'a portion of matter of constant composition, composed of molecular entities of the same type or of different types' (source: [ChEBI:59999](https://www.ebi.ac.uk/chebi/searchId.do?chebiId=59999)).
#[derive()]
pub struct ChemicalSubstance {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ChemicalSubstance {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A television station.
#[derive()]
pub struct TelevisionStation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TelevisionStation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of consuming written content.
#[derive()]
pub struct ReadAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReadAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates types (or dimensions) of a person's body measurements, for example for fitting of clothes.
#[derive()]
pub struct BodyMeasurementTypeEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BodyMeasurementTypeEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A demand entity represents the public, not necessarily binding, not necessarily exclusive, announcement by an organization or person to seek a certain type of goods or services. For describing demand using this type, the very same properties used for Offer apply.
#[derive()]
pub struct Demand {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Demand {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A creative work with a visual storytelling format intended to be viewed online, particularly on mobile devices.
#[derive()]
pub struct AmpStory {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AmpStory {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A single episode of a podcast series.
#[derive()]
pub struct PodcastEpisode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PodcastEpisode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of transferring money from one place to another place. This may occur electronically or physically.
#[derive()]
pub struct MoneyTransfer {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MoneyTransfer {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Business event.
#[derive()]
pub struct BusinessEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BusinessEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A pharmacy or drugstore.
#[derive()]
pub struct Pharmacy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Pharmacy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A room is a distinguishable space within a structure, usually separated from other spaces by interior walls (source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/Room">http://en.wikipedia.org/wiki/Room</a>).
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
/// 
#[derive()]
pub struct Room {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Room {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A guideline contraindication that designates a process as harmful and where quality of the data supporting the contraindication is sound.
#[derive()]
pub struct MedicalGuidelineContraindication {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalGuidelineContraindication {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A tourist destination. In principle any [[Place]] can be a [[TouristDestination]] from a [[City]], Region or [[Country]] to an [[AmusementPark]] or [[Hotel]]. This Type can be used on its own to describe a general [[TouristDestination]], or be used as an [[additionalType]] to add tourist relevant properties to any other [[Place]].  A [[TouristDestination]] is defined as a [[Place]] that contains, or is colocated with, one or more [[TouristAttraction]]s, often linked by a similar theme or interest to a particular [[touristType]]. The [UNWTO](http://www2.unwto.org/) defines Destination (main destination of a tourism trip) as the place visited that is central to the decision to take the trip.
///   (See examples below.)
#[derive()]
pub struct TouristDestination {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TouristDestination {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A ski resort.
#[derive()]
pub struct SkiResort {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SkiResort {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A shopping center or mall.
#[derive()]
pub struct ShoppingCenter {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ShoppingCenter {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A ShippingRateSettings represents re-usable pieces of shipping information. It is designed for publication on an URL that may be referenced via the [[shippingSettingsLink]] property of an [[OfferShippingDetails]]. Several occurrences can be published, distinguished and matched (i.e. identified/referenced) by their different values for [[shippingLabel]].
#[derive()]
pub struct ShippingRateSettings {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ShippingRateSettings {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A tourist trip. A created itinerary of visits to one or more places of interest ([[TouristAttraction]]/[[TouristDestination]]) often linked by a similar theme, geographic area, or interest to a particular [[touristType]]. The [UNWTO](http://www2.unwto.org/) defines tourism trip as the Trip taken by visitors.
///   (See examples below.)
#[derive()]
pub struct TouristTrip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TouristTrip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A thesis or dissertation document submitted in support of candidature for an academic degree or professional qualification.
#[derive()]
pub struct Thesis {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Thesis {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Classification of the album by its type of content: soundtrack, live album, studio album, etc.
#[derive()]
pub struct MusicAlbumProductionType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicAlbumProductionType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Literary event.
#[derive()]
pub struct LiteraryEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LiteraryEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Residence type: Single-family home.
#[derive()]
pub struct SingleFamilyResidence {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SingleFamilyResidence {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// One of the sections into which a book is divided. A chapter usually has a section number or a name.
#[derive()]
pub struct Chapter {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Chapter {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of producing/preparing food.
#[derive()]
pub struct CookAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CookAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A short TV or radio program or a segment/part of a program.
#[derive()]
pub struct Clip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Clip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A ProductGroup represents a group of [[Product]]s that vary only in certain well-described ways, such as by [[size]], [[color]], [[material]] etc.
/// 
/// While a ProductGroup itself is not directly offered for sale, the various varying products that it represents can be. The ProductGroup serves as a prototype or template, standing in for all of the products who have an [[isVariantOf]] relationship to it. As such, properties (including additional types) can be applied to the ProductGroup to represent characteristics shared by each of the (possibly very many) variants. Properties that reference a ProductGroup are not included in this mechanism; neither are the following specific properties [[variesBy]], [[hasVariant]], [[url]]. 
#[derive()]
pub struct ProductGroup {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ProductGroup {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A casino.
#[derive()]
pub struct Casino {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Casino {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A profession, may involve prolonged training and/or a formal qualification.
#[derive()]
pub struct Occupation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Occupation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A person (alive, dead, undead, or fictional).
#[derive()]
pub struct Person {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Person {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An emergency service, such as a fire station or ER.
#[derive()]
pub struct EmergencyService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EmergencyService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A muscle is an anatomical structure consisting of a contractile form of tissue that animals use to effect movement.
#[derive()]
pub struct Muscle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Muscle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A notary.
#[derive()]
pub struct Notary {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Notary {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The most generic type of item.
#[derive()]
pub struct Thing {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Thing {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A kind of lodging business that focuses on renting single properties for limited time.
#[derive()]
pub struct VacationRental {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VacationRental {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A distillery.
#[derive()]
pub struct Distillery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Distillery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A structured representation of food or drink items available from a FoodEstablishment.
#[derive()]
pub struct Menu {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Menu {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A mosque.
#[derive()]
pub struct Mosque {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Mosque {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Scheduling future actions, events, or tasks.\n\nRelated actions:\n\n* [[ReserveAction]]: Unlike ReserveAction, ScheduleAction allocates future actions (e.g. an event, a task, etc) towards a time slot / spatial allocation.
#[derive()]
pub struct ScheduleAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ScheduleAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A furniture store.
#[derive()]
pub struct FurnitureStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FurnitureStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A computer store.
#[derive()]
pub struct ComputerStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ComputerStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A unique instance of a television BroadcastService on a CableOrSatelliteService lineup.
#[derive()]
pub struct TelevisionChannel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TelevisionChannel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A prion is an infectious agent composed of protein in a misfolded form.
#[derive()]
pub struct Prion {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Prion {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An EducationalAudience.
#[derive()]
pub struct EducationalAudience {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EducationalAudience {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A subclass of OrganizationRole used to describe employee relationships.
#[derive()]
pub struct EmployeeRole {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EmployeeRole {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An elementary school.
#[derive()]
pub struct ElementarySchool {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ElementarySchool {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A terminal for boats, ships, and other water vessels.
#[derive()]
pub struct BoatTerminal {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BoatTerminal {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Social event.
#[derive()]
pub struct SocialEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SocialEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A tool used (but not consumed) when performing instructions for how to achieve a result.
#[derive()]
pub struct HowToTool {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HowToTool {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// EnergyConsumptionDetails represents information related to the energy efficiency of a product that consumes energy. The information that can be provided is based on international regulations such as for example [EU directive 2017/1369](https://eur-lex.europa.eu/eli/reg/2017/1369/oj) for energy labeling and the [Energy labeling rule](https://www.ftc.gov/enforcement/rules/rulemaking-regulatory-reform-proceedings/energy-water-use-labeling-consumer) under the Energy Policy and Conservation Act (EPCA) in the US.
#[derive()]
pub struct EnergyConsumptionDetails {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EnergyConsumptionDetails {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A discrete unit of inheritance which affects one or more biological traits (Source: [https://en.wikipedia.org/wiki/Gene](https://en.wikipedia.org/wiki/Gene)). Examples include FOXP2 (Forkhead box protein P2), SCARNA21 (small Cajal body-specific RNA 21), A- (agouti genotype).
#[derive()]
pub struct Gene {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Gene {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A process of care used in either a diagnostic, therapeutic, preventive or palliative capacity that relies on invasive (surgical), non-invasive, or other techniques.
#[derive()]
pub struct MedicalProcedure {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalProcedure {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An electronics store.
#[derive()]
pub struct ElectronicsStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ElectronicsStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Categories of physical activity, organized by physiologic classification.
#[derive()]
pub struct PhysicalActivityCategory {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PhysicalActivityCategory {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A landform or physical feature.  Landform elements include mountains, plains, lakes, rivers, seascape and oceanic waterbody interface features such as bays, peninsulas, seas and so forth, including sub-aqueous terrain features such as submersed mountain ranges, volcanoes, and the great ocean basins.
#[derive()]
pub struct Landform {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Landform {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A condition or factor that serves as a reason to withhold a certain medical therapy. Contraindications can be absolute (there are no reasonable circumstances for undertaking a course of action) or relative (the patient is at higher risk of complications, but these risks may be outweighed by other considerations or mitigated by other measures).
#[derive()]
pub struct MedicalContraindication {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalContraindication {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A performance group, such as a band, an orchestra, or a circus.
#[derive()]
pub struct PerformingGroup {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PerformingGroup {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Category Code.
#[derive()]
pub struct CategoryCode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CategoryCode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Used to describe a seat, such as a reserved seat in an event reservation.
#[derive()]
pub struct Seat {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Seat {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any condition of the human body that affects the normal functioning of a person, whether physically or mentally. Includes diseases, injuries, disabilities, disorders, syndromes, etc.
#[derive()]
pub struct MedicalCondition {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalCondition {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// EventStatusType is an enumeration type whose instances represent several states that an Event may be in.
#[derive()]
pub struct EventStatusType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EventStatusType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A legislative building&#x2014;for example, the state capitol.
#[derive()]
pub struct LegislativeBuilding {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LegislativeBuilding {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of notifying someone of information pertinent to them, with no expectation of a response.
#[derive()]
pub struct InformAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InformAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any physical manifestation of a person's medical condition discoverable by objective diagnostic tests or physical examination.
#[derive()]
pub struct MedicalSign {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalSign {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumeration(s) for use with [[measurementMethod]].
#[derive()]
pub struct MeasurementMethodEnum {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MeasurementMethodEnum {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The geographic coordinates of a place or event.
#[derive()]
pub struct GeoCoordinates {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GeoCoordinates {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A single message from a sender to one or more organizations or people.
#[derive()]
pub struct Message {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Message {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of accomplishing something via previous efforts. It is an instantaneous action rather than an ongoing process.
#[derive()]
pub struct AchieveAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AchieveAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
#[derive()]
pub struct UserBlocks {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserBlocks {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of ingesting information/resources/food.
#[derive()]
pub struct ConsumeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ConsumeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any medical intervention designed to prevent, treat, and cure human diseases and medical conditions, including both curative and palliative therapies. Medical therapies are typically processes of care relying upon pharmacotherapy, behavioral therapy, supportive therapy (with fluid or nutrition for example), or detoxification (e.g. hemodialysis) aimed at improving or preventing a health condition.
#[derive()]
pub struct MedicalTherapy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalTherapy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A reservation for air travel.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
#[derive()]
pub struct FlightReservation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FlightReservation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A LegalService is a business that provides legally-oriented services, advice and representation, e.g. law firms.\n\nAs a [[LocalBusiness]] it can be described as a [[provider]] of one or more [[Service]]\(s).
#[derive()]
pub struct LegalService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LegalService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any medical imaging modality typically used for diagnostic purposes. Enumerated type.
#[derive()]
pub struct MedicalImagingTechnique {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalImagingTechnique {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A musical group, such as a band, an orchestra, or a choir. Can also be a solo musician.
#[derive()]
pub struct MusicGroup {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicGroup {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of participating in an exchange of goods and services for monetary compensation. An agent trades an object, product or service with a participant in exchange for a one time or periodic payment.
#[derive()]
pub struct TradeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TradeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A government building.
#[derive()]
pub struct GovernmentBuilding {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GovernmentBuilding {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A value indicating a steering position.
#[derive()]
pub struct SteeringPositionValue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SteeringPositionValue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any biological, chemical, or biochemical thing. For example: a protein; a gene; a chemical; a synthetic chemical.
#[derive()]
pub struct BioChemEntity {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BioChemEntity {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A toy store.
#[derive()]
pub struct ToyStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ToyStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An offer to transfer some rights to an item or to provide a service — for example, an offer to sell tickets to an event, to rent the DVD of a movie, to stream a TV show over the internet, to repair a motorcycle, or to loan a book.\n\nNote: As the [[businessFunction]] property, which identifies the form of offer (e.g. sell, lease, repair, dispose), defaults to http://purl.org/goodrelations/v1#Sell; an Offer without a defined businessFunction value can be assumed to be an offer to sell.\n\nFor [GTIN](http://www.gs1.org/barcodes/technical/idkeys/gtin)-related fields, see [Check Digit calculator](http://www.gs1.org/barcodes/support/check_digit_calculator) and [validation guide](http://www.gs1us.org/resources/standards/gtin-validation-guide) from [GS1](http://www.gs1.org/).
#[derive()]
pub struct Offer {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Offer {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A high school.
#[derive()]
pub struct HighSchool {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HighSchool {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[NewsArticle]] and [[CriticReview]] providing a professional critic's assessment of a service, product, performance, or artistic or literary work.
#[derive()]
pub struct ReviewNewsArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReviewNewsArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Pathogenic fungus.
#[derive()]
pub struct Fungus {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Fungus {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates common size groups for various product categories.
#[derive()]
pub struct SizeGroupEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SizeGroupEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A service which provides access to media programming like TV or radio. Access may be via cable or satellite.
#[derive()]
pub struct CableOrSatelliteService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CableOrSatelliteService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A food-related business.
#[derive()]
pub struct FoodEstablishment {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FoodEstablishment {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Web page type: Search results page.
#[derive()]
pub struct SearchResultsPage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SearchResultsPage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A reservation for lodging at a hotel, motel, inn, etc.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations.
#[derive()]
pub struct LodgingReservation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LodgingReservation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A college, university, or other third-level educational institution.
#[derive()]
pub struct CollegeOrUniversity {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CollegeOrUniversity {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A medical test performed by a laboratory that typically involves examination of a tissue sample by a pathologist.
#[derive()]
pub struct PathologyTest {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PathologyTest {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of an agent communicating (service provider, social media, etc) their arrival by registering/confirming for a previously reserved service (e.g. flight check-in) or at a place (e.g. hotel), possibly resulting in a result (boarding pass, etc).\n\nRelated actions:\n\n* [[CheckOutAction]]: The antonym of CheckInAction.\n* [[ArriveAction]]: Unlike ArriveAction, CheckInAction implies that the agent is informing/confirming the start of a previously reserved service.\n* [[ConfirmAction]]: Unlike ConfirmAction, CheckInAction implies that the agent is informing/confirming the *start* of a previously reserved service rather than its validity/existence.
#[derive()]
pub struct CheckInAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CheckInAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A set of products (either [[ProductGroup]]s or specific variants) that are listed together e.g. in an [[Offer]].
#[derive()]
pub struct ProductCollection {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ProductCollection {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of an agent communicating (service provider, social media, etc) their departure of a previously reserved service (e.g. flight check-in) or place (e.g. hotel).\n\nRelated actions:\n\n* [[CheckInAction]]: The antonym of CheckOutAction.\n* [[DepartAction]]: Unlike DepartAction, CheckOutAction implies that the agent is informing/confirming the end of a previously reserved service.\n* [[CancelAction]]: Unlike CancelAction, CheckOutAction implies that the agent is informing/confirming the end of a previously reserved service.
#[derive()]
pub struct CheckOutAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CheckOutAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A taxi.
#[derive()]
pub struct Taxi {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Taxi {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[NewsArticle]] expressing an open call by a [[NewsMediaOrganization]] asking the public for input, insights, clarifications, anecdotes, documentation, etc., on an issue, for reporting purposes.
#[derive()]
pub struct AskPublicNewsArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AskPublicNewsArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Computer programming source code. Example: Full (compile ready) solutions, code snippet samples, scripts, templates.
#[derive()]
pub struct Code {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Code {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A word, name, acronym, phrase, etc. with a formal definition. Often used in the context of category or subject classification, glossaries or dictionaries, product or creative work types, etc. Use the name property for the term being defined, use termCode if the term has an alpha-numeric code allocated, use description to provide the definition of the term.
#[derive()]
pub struct DefinedTerm {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DefinedTerm {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A day spa.
#[derive()]
pub struct DaySpa {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DaySpa {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A trip on a commercial ferry line.
#[derive()]
pub struct BoatTrip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BoatTrip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of consuming dynamic/moving visual content.
#[derive()]
pub struct WatchAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WatchAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of asking someone to attend an event. Reciprocal of RsvpAction.
#[derive()]
pub struct InviteAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InviteAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The artwork on the outer surface of a CreativeWork.
#[derive()]
pub struct CoverArt {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CoverArt {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of expressing a preference from a fixed/finite/structured set of choices/options.
#[derive()]
pub struct VoteAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VoteAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of participating in exertive activity for the purposes of improving health and fitness.
#[derive()]
pub struct ExerciseAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ExerciseAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of giving money voluntarily to a beneficiary in recognition of services rendered.
#[derive()]
pub struct TipAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TipAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Catholic church.
#[derive()]
pub struct CatholicChurch {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CatholicChurch {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Format of this release (the type of recording media used, i.e. compact disc, digital media, LP, etc.).
#[derive()]
pub struct MusicReleaseFormatType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicReleaseFormatType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A courthouse.
#[derive()]
pub struct Courthouse {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Courthouse {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// [[Recommendation]] is a type of [[Review]] that suggests or proposes something as the best option or best course of action. Recommendations may be for products or services, or other concrete things, as in the case of a ranked list or product guide. A [[Guide]] may list multiple recommendations for different categories. For example, in a [[Guide]] about which TVs to buy, the author may have several [[Recommendation]]s.
#[derive()]
pub struct Recommendation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Recommendation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// [[Guide]] is a page or article that recommends specific products or services, or aspects of a thing for a user to consider. A [[Guide]] may represent a Buying Guide and detail aspects of products or services for a user to consider. A [[Guide]] may represent a Product Guide and recommend specific products or services. A [[Guide]] may represent a Ranked List and recommend specific products or services with ranking.
#[derive()]
pub struct Guide {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Guide {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A musical composition.
#[derive()]
pub struct MusicComposition {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicComposition {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A motorcycle or motorbike is a single-track, two-wheeled motor vehicle.
#[derive()]
pub struct Motorcycle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Motorcycle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A hardware store.
#[derive()]
pub struct HardwareStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HardwareStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of giving money to a seller in exchange for goods or services rendered. An agent buys an object, product, or service from a seller for a price. Reciprocal of SellAction.
#[derive()]
pub struct BuyAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BuyAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// ShippingDeliveryTime provides various pieces of information about delivery times for shipping.
#[derive()]
pub struct ShippingDeliveryTime {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ShippingDeliveryTime {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// CreativeWorkSeries dedicated to radio broadcast and associated online delivery.
#[derive()]
pub struct RadioSeries {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RadioSeries {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A moving company.
#[derive()]
pub struct MovingCompany {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MovingCompany {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[LiveBlogPosting]] is a [[BlogPosting]] intended to provide a rolling textual coverage of an ongoing event through continuous updates.
#[derive()]
pub struct LiveBlogPosting {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LiveBlogPosting {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A patient is any person recipient of health care services.
#[derive()]
pub struct Patient {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Patient {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Anatomical features that can be observed by sight (without dissection), including the form and proportions of the human body as well as surface landmarks that correspond to deeper subcutaneous structures. Superficial anatomy plays an important role in sports medicine, phlebotomy, and other medical specialties as underlying anatomical structures can be identified through surface palpation. For example, during back surgery, superficial anatomy can be used to palpate and count vertebrae to find the site of incision. Or in phlebotomy, superficial anatomy can be used to locate an underlying vein; for example, the median cubital vein can be located by palpating the borders of the cubital fossa (such as the epicondyles of the humerus) and then looking for the superficial signs of the vein, such as size, prominence, ability to refill after depression, and feel of surrounding tissue support. As another example, in a subluxation (dislocation) of the glenohumeral joint, the bony structure becomes pronounced with the deltoid muscle failing to cover the glenohumeral joint allowing the edges of the scapula to be superficially visible. Here, the superficial anatomy is the visible edges of the scapula, implying the underlying dislocation of the joint (the related anatomical structure).
#[derive()]
pub struct SuperficialAnatomy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SuperficialAnatomy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An employment agency.
#[derive()]
pub struct EmploymentAgency {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EmploymentAgency {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A suite in a hotel or other public accommodation, denotes a class of luxury accommodations, the key feature of which is multiple rooms (source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/Suite_(hotel)">http://en.wikipedia.org/wiki/Suite_(hotel)</a>).
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
/// 
#[derive()]
pub struct Suite {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Suite {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A structured value representing exchange rate.
#[derive()]
pub struct ExchangeRateSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ExchangeRateSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A schedule defines a repeating time period used to describe a regularly occurring [[Event]]. At a minimum a schedule will specify [[repeatFrequency]] which describes the interval between occurrences of the event. Additional information can be provided to specify the schedule more precisely.
///       This includes identifying the day(s) of the week or month when the recurring event will take place, in addition to its start and end time. Schedules may also
///       have start and end dates to indicate when they are active, e.g. to define a limited calendar of events.
#[derive()]
pub struct Schedule {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Schedule {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A city or town.
#[derive()]
pub struct City {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for City {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Role that represents a Web link, e.g. as expressed via the 'url' property. Its linkRelationship property can indicate URL-based and plain textual link types, e.g. those in IANA link registry or others such as 'amphtml'. This structure provides a placeholder where details from HTML's link element can be represented outside of HTML, e.g. in JSON-LD feeds.
#[derive()]
pub struct LinkRole {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LinkRole {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A bakery.
#[derive()]
pub struct Bakery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Bakery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An [[EmployerReview]] is a review of an [[Organization]] regarding its role as an employer, written by a current or former employee of that organization.
#[derive()]
pub struct EmployerReview {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EmployerReview {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of committing to/adopting an object.\n\nRelated actions:\n\n* [[RejectAction]]: The antonym of AcceptAction.
#[derive()]
pub struct AcceptAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AcceptAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A TV episode which can be part of a series or season.
#[derive()]
pub struct TVEpisode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TVEpisode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of forming one's opinion, reaction or sentiment.
#[derive()]
pub struct AssessAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AssessAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Web page type: Contact page.
#[derive()]
pub struct ContactPage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ContactPage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Lists or enumerations dealing with status types.
#[derive()]
pub struct StatusEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for StatusEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Web page type: Media gallery page. A mixed-media page that can contain media such as images, videos, and other multimedia.
#[derive()]
pub struct MediaGallery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MediaGallery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A file composed primarily of text.
#[derive()]
pub struct TextDigitalDocument {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TextDigitalDocument {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of registering to be a user of a service, product or web page.\n\nRelated actions:\n\n* [[JoinAction]]: Unlike JoinAction, RegisterAction implies you are registering to be a user of a service, *not* a group/team of people.\n* [[FollowAction]]: Unlike FollowAction, RegisterAction doesn't imply that the agent is expecting to poll for updates from the object.\n* [[SubscribeAction]]: Unlike SubscribeAction, RegisterAction doesn't imply that the agent is expecting updates from the object.
#[derive()]
pub struct RegisterAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RegisterAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Original definition: "provider of professional services."\n\nThe general [[ProfessionalService]] type for local businesses was deprecated due to confusion with [[Service]]. For reference, the types that it included were: [[Dentist]],
///         [[AccountingService]], [[Attorney]], [[Notary]], as well as types for several kinds of [[HomeAndConstructionBusiness]]: [[Electrician]], [[GeneralContractor]],
///         [[HousePainter]], [[Locksmith]], [[Plumber]], [[RoofingContractor]]. [[LegalService]] was introduced as a more inclusive supertype of [[Attorney]].
#[derive()]
pub struct ProfessionalService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ProfessionalService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Workers Union (also known as a Labor Union, Labour Union, or Trade Union) is an organization that promotes the interests of its worker members by collectively bargaining with management, organizing, and political lobbying.
#[derive()]
pub struct WorkersUnion {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WorkersUnion {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A sporting goods store.
#[derive()]
pub struct SportingGoodsStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SportingGoodsStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An educational or occupational credential. A diploma, academic degree, certification, qualification, badge, etc., that may be awarded to a person or other entity that meets the requirements defined by the credentialer.
#[derive()]
pub struct EducationalOccupationalCredential {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EducationalOccupationalCredential {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Sports event.
#[derive()]
pub struct SportsEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SportsEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An aggregate rating of an Organization related to its role as an employer.
#[derive()]
pub struct EmployerAggregateRating {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EmployerAggregateRating {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A table on a Web page.
#[derive()]
pub struct Table {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Table {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A delivery service through which radio content is provided via broadcast over the air or online.
#[derive()]
pub struct RadioBroadcastService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RadioBroadcastService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A collection or bound volume of maps, charts, plates or tables, physical or in media form illustrating any subject.
#[derive()]
pub struct Atlas {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Atlas {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A bus (also omnibus or autobus) is a road vehicle designed to carry passengers. Coaches are luxury busses, usually in service for long distance travel.
#[derive()]
pub struct BusOrCoach {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BusOrCoach {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Sales event.
#[derive()]
pub struct SaleEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SaleEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A technical article - Example: How-to (task) topics, step-by-step, procedural troubleshooting, specifications, etc.
#[derive()]
pub struct TechArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TechArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A grant, typically financial or otherwise quantifiable, of resources. Typically a [[funder]] sponsors some [[MonetaryAmount]] to an [[Organization]] or [[Person]],
///     sometimes not necessarily via a dedicated or long-lived [[Project]], resulting in one or more outputs, or [[fundedItem]]s. For financial sponsorship, indicate the [[funder]] of a [[MonetaryGrant]]. For non-financial support, indicate [[sponsor]] of [[Grant]]s of resources (e.g. office space).
/// 
/// Grants support  activities directed towards some agreed collective goals, often but not always organized as [[Project]]s. Long-lived projects are sometimes sponsored by a variety of grants over time, but it is also common for a project to be associated with a single grant.
/// 
/// The amount of a [[Grant]] is represented using [[amount]] as a [[MonetaryAmount]].
///     
#[derive()]
pub struct Grant {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Grant {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A governmental organization or agency.
#[derive()]
pub struct GovernmentOrganization {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GovernmentOrganization {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any object used in a medical capacity, such as to diagnose or treat a patient.
#[derive()]
pub struct MedicalDevice {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalDevice {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of expressing a desire about the object. An agent wants an object.
#[derive()]
pub struct WantAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WantAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerations related to health and the practice of medicine: A concept that is used to attribute a quality to another concept, as a qualifier, a collection of items or a listing of all of the elements of a set in medicine practice.
#[derive()]
pub struct MedicalEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of deliberately creating/producing/generating/building a result out of the agent.
#[derive()]
pub struct CreateAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CreateAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The most generic kind of creative work, including books, movies, photographs, software programs, etc.
#[derive()]
pub struct CreativeWork {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CreativeWork {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Service to transfer funds from a person or organization to a beneficiary person or organization.
#[derive()]
pub struct PaymentService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PaymentService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An audiobook.
#[derive()]
pub struct Audiobook {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Audiobook {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A motel.
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
/// 
#[derive()]
pub struct Motel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Motel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An AnalysisNewsArticle is a [[NewsArticle]] that, while based on factual reporting, incorporates the expertise of the author/producer, offering interpretations and conclusions.
#[derive()]
pub struct AnalysisNewsArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AnalysisNewsArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates common size systems specific for wearable products
#[derive()]
pub struct WearableSizeSystemEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WearableSizeSystemEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A screening of a movie or other video.
#[derive()]
pub struct ScreeningEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ScreeningEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates energy efficiency levels (also known as "classes" or "ratings") and certifications that are part of several international energy efficiency standards.
#[derive()]
pub struct EnergyEfficiencyEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EnergyEfficiencyEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A defence establishment, such as an army or navy base.
#[derive()]
pub struct DefenceEstablishment {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DefenceEstablishment {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An [[Article]] that an external entity has paid to place or to produce to its specifications. Includes [advertorials](https://en.wikipedia.org/wiki/Advertorial), sponsored content, native advertising and other paid content.
#[derive()]
pub struct AdvertiserContentArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AdvertiserContentArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// NonprofitType enumerates several kinds of official non-profit types of which a non-profit organization can be.
#[derive()]
pub struct NonprofitType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for NonprofitType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A list of possible conditions for the item.
#[derive()]
pub struct OfferItemCondition {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OfferItemCondition {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Children's event.
#[derive()]
pub struct ChildrensEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ChildrensEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The footer section of the page.
#[derive()]
pub struct WPFooter {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WPFooter {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A direction indicating a single action to do in the instructions for how to achieve a result.
#[derive()]
pub struct HowToDirection {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HowToDirection {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// OfferShippingDetails represents information about shipping destinations.
/// 
/// Multiple of these entities can be used to represent different shipping rates for different destinations:
/// 
/// One entity for Alaska/Hawaii. A different one for continental US. A different one for all France.
/// 
/// Multiple of these entities can be used to represent different shipping costs and delivery times.
/// 
/// Two entities that are identical but differ in rate and time:
/// 
/// E.g. Cheaper and slower: $5 in 5-7 days
/// or Fast and expensive: $15 in 1-2 days.
#[derive()]
pub struct OfferShippingDetails {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OfferShippingDetails {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An event happening at a certain time and location, such as a concert, lecture, or festival. Ticketing information may be added via the [[offers]] property. Repeated events may be structured as separate Event objects.
#[derive()]
pub struct Event {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Event {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent quotes/estimates/appraises an object/product/service with a price at a location/store.
#[derive()]
pub struct QuoteAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for QuoteAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A SpecialAnnouncement combines a simple date-stamped textual information update
///       with contextualized Web links and other structured data.  It represents an information update made by a
///       locally-oriented organization, for example schools, pharmacies, healthcare providers,  community groups, police,
///       local government.
/// 
/// For work in progress guidelines on Coronavirus-related markup see [this doc](https://docs.google.com/document/d/14ikaGCKxo50rRM7nvKSlbUpjyIk2WMQd3IkB1lItlrM/edit#).
/// 
/// The motivating scenario for SpecialAnnouncement is the [Coronavirus pandemic](https://en.wikipedia.org/wiki/2019%E2%80%9320_coronavirus_pandemic), and the initial vocabulary is oriented to this urgent situation. Schema.org
/// expect to improve the markup iteratively as it is deployed and as feedback emerges from use. In addition to our
/// usual [Github entry](https://github.com/schemaorg/schemaorg/issues/2490), feedback comments can also be provided in [this document](https://docs.google.com/document/d/1fpdFFxk8s87CWwACs53SGkYv3aafSxz_DTtOQxMrBJQ/edit#).
/// 
/// 
/// While this schema is designed to communicate urgent crisis-related information, it is not the same as an emergency warning technology like [CAP](https://en.wikipedia.org/wiki/Common_Alerting_Protocol), although there may be overlaps. The intent is to cover
/// the kinds of everyday practical information being posted to existing websites during an emergency situation.
/// 
/// Several kinds of information can be provided:
/// 
/// We encourage the provision of "name", "text", "datePosted", "expires" (if appropriate), "category" and
/// "url" as a simple baseline. It is important to provide a value for "category" where possible, most ideally as a well known
/// URL from Wikipedia or Wikidata. In the case of the 2019-2020 Coronavirus pandemic, this should be "https://en.wikipedia.org/w/index.php?title=2019-20\_coronavirus\_pandemic" or "https://www.wikidata.org/wiki/Q81068910".
/// 
/// For many of the possible properties, values can either be simple links or an inline description, depending on whether a summary is available. For a link, provide just the URL of the appropriate page as the property's value. For an inline description, use a [[WebContent]] type, and provide the url as a property of that, alongside at least a simple "[[text]]" summary of the page. It is
/// unlikely that a single SpecialAnnouncement will need all of the possible properties simultaneously.
/// 
/// We expect that in many cases the page referenced might contain more specialized structured data, e.g. contact info, [[openingHours]], [[Event]], [[FAQPage]] etc. By linking to those pages from a [[SpecialAnnouncement]] you can help make it clearer that the events are related to the situation (e.g. Coronavirus) indicated by the [[category]] property of the [[SpecialAnnouncement]].
/// 
/// Many [[SpecialAnnouncement]]s will relate to particular regions and to identifiable local organizations. Use [[spatialCoverage]] for the region, and [[announcementLocation]] to indicate specific [[LocalBusiness]]es and [[CivicStructure]]s. If the announcement affects both a particular region and a specific location (for example, a library closure that serves an entire region), use both [[spatialCoverage]] and [[announcementLocation]].
/// 
/// The [[about]] property can be used to indicate entities that are the focus of the announcement. We now recommend using [[about]] only
/// for representing non-location entities (e.g. a [[Course]] or a [[RadioStation]]). For places, use [[announcementLocation]] and [[spatialCoverage]]. Consumers of this markup should be aware that the initial design encouraged the use of [[about]] for locations too.
/// 
/// The basic content of [[SpecialAnnouncement]] is similar to that of an [RSS](https://en.wikipedia.org/wiki/RSS) or [Atom](https://en.wikipedia.org/wiki/Atom_(Web_standard)) feed. For publishers without such feeds, basic feed-like information can be shared by posting
/// [[SpecialAnnouncement]] updates in a page, e.g. using JSON-LD. For sites with Atom/RSS functionality, you can point to a feed
/// with the [[webFeed]] property. This can be a simple URL, or an inline [[DataFeed]] object, with [[encodingFormat]] providing
/// media type information, e.g. "application/rss+xml" or "application/atom+xml".
/// 
#[derive()]
pub struct SpecialAnnouncement {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SpecialAnnouncement {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The term "story" is any indivisible, re-printable
///     	unit of a comic, including the interior stories, covers, and backmatter. Most
///     	comics have at least two stories: a cover (ComicCoverArt) and an interior story.
#[derive()]
pub struct ComicStory {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ComicStory {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Car repair business.
#[derive()]
pub struct AutoRepair {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AutoRepair {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerated categories of medical drug costs.
#[derive()]
pub struct DrugCostCategory {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DrugCostCategory {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A permission for a particular person or group to access a particular file.
#[derive()]
pub struct DigitalDocumentPermission {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DigitalDocumentPermission {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A department store.
#[derive()]
pub struct DepartmentStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DepartmentStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A type of blood vessel that specifically carries lymph fluid unidirectionally toward the heart.
#[derive()]
pub struct LymphaticVessel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LymphaticVessel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An organization such as a school, NGO, corporation, club, etc.
#[derive()]
pub struct Organization {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Organization {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A particular physical business or branch of an organization. Examples of LocalBusiness include a restaurant, a particular branch of a restaurant chain, a branch of a bank, a medical practice, a club, a bowling alley, etc.
#[derive()]
pub struct LocalBusiness {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LocalBusiness {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A hospital.
#[derive()]
pub struct Hospital {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Hospital {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
#[derive()]
pub struct UserPlays {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserPlays {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A structured value indicating the quantity, unit of measurement, and business function of goods included in a bundle offer.
#[derive()]
pub struct TypeAndQuantityNode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TypeAndQuantityNode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// All or part of a [[Dataset]] in downloadable form. 
#[derive()]
pub struct DataDownload {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DataDownload {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A web page element, like a table or an image.
#[derive()]
pub struct WebPageElement {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WebPageElement {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A group of multiple reservations with common values for all sub-reservations.
#[derive()]
pub struct ReservationPackage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReservationPackage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Natural languages such as Spanish, Tamil, Hindi, English, etc. Formal language code tags expressed in [BCP 47](https://en.wikipedia.org/wiki/IETF_language_tag) can be used via the [[alternateName]] property. The Language type previously also covered programming languages such as Scheme and Lisp, which are now best represented using [[ComputerLanguage]].
#[derive()]
pub struct Language {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Language {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// One of the continents (for example, Europe or Africa).
#[derive()]
pub struct Continent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Continent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An adult entertainment establishment.
#[derive()]
pub struct AdultEntertainment {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AdultEntertainment {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An EndorsementRating is a rating that expresses some level of endorsement, for example inclusion in a "critic's pick" blog, a
/// "Like" or "+1" on a social network. It can be considered the [[result]] of an [[EndorseAction]] in which the [[object]] of the action is rated positively by
/// some [[agent]]. As is common elsewhere in schema.org, it is sometimes more useful to describe the results of such an action without explicitly describing the [[Action]].
/// 
/// An [[EndorsementRating]] may be part of a numeric scale or organized system, but this is not required: having an explicit type for indicating a positive,
/// endorsement rating is particularly useful in the absence of numeric scales as it helps consumers understand that the rating is broadly positive.
/// 
#[derive()]
pub struct EndorsementRating {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EndorsementRating {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A specific payment status. For example, PaymentDue, PaymentComplete, etc.
#[derive()]
pub struct PaymentStatusType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PaymentStatusType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Target audiences for medical web pages.
#[derive()]
pub struct MedicalAudience {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalAudience {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A service provided by a government organization, e.g. food stamps, veterans benefits, etc.
#[derive()]
pub struct GovernmentService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GovernmentService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Properties that take Energy as values are of the form '&lt;Number&gt; &lt;Energy unit of measure&gt;'.
#[derive()]
pub struct Energy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Energy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of participating in performance arts.
#[derive()]
pub struct PerformAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PerformAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Classes of agents or pathogens that transmit infectious diseases. Enumerated type.
#[derive()]
pub struct InfectiousAgentClass {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InfectiousAgentClass {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A MusicRelease is a specific release of a music album.
#[derive()]
pub struct MusicRelease {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicRelease {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A nightclub or discotheque.
#[derive()]
pub struct NightClub {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for NightClub {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of authoring written creative content.
#[derive()]
pub struct WriteAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WriteAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// This type covers computer programming languages such as Scheme and Lisp, as well as other language-like computer representations. Natural languages are best represented with the [[Language]] type.
#[derive()]
pub struct ComputerLanguage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ComputerLanguage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The frequency in MHz and the modulation used for a particular BroadcastService.
#[derive()]
pub struct BroadcastFrequencySpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BroadcastFrequencySpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The kind of release which this album is: single, EP or album.
#[derive()]
pub struct MusicAlbumReleaseType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicAlbumReleaseType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A type of financial product that typically requires the client to transfer funds to a financial service in return for potential beneficial financial return.
#[derive()]
pub struct InvestmentOrDeposit {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InvestmentOrDeposit {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A placeholder for multiple similar products of the same kind.
#[derive()]
pub struct SomeProducts {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SomeProducts {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of marrying a person.
#[derive()]
pub struct MarryAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MarryAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The causative agent(s) that are responsible for the pathophysiologic process that eventually results in a medical condition, symptom or sign. In this schema, unless otherwise specified this is meant to be the proximate cause of the medical condition, symptom or sign. The proximate cause is defined as the causative agent that most directly results in the medical condition, symptom or sign. For example, the HIV virus could be considered a cause of AIDS. Or in a diagnostic context, if a patient fell and sustained a hip fracture and two days later sustained a pulmonary embolism which eventuated in a cardiac arrest, the cause of the cardiac arrest (the proximate cause) would be the pulmonary embolism and not the fall. Medical causes can include cardiovascular, chemical, dermatologic, endocrine, environmental, gastroenterologic, genetic, hematologic, gynecologic, iatrogenic, infectious, musculoskeletal, neurologic, nutritional, obstetric, oncologic, otolaryngologic, pharmacologic, psychiatric, pulmonary, renal, rheumatologic, toxic, traumatic, or urologic causes; medical conditions can be causes as well.
#[derive()]
pub struct MedicalCause {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalCause {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Server that provides game interaction in a multiplayer game.
#[derive()]
pub struct GameServer {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GameServer {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A CDCPMDRecord is a data structure representing a record in a CDC tabular data format
///       used for hospital data reporting. See [documentation](/docs/cdc-covid.html) for details, and the linked CDC materials for authoritative
///       definitions used as the source here.
///       
#[derive()]
pub struct CDCPMDRecord {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CDCPMDRecord {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The maximum dosing schedule considered safe for a drug or supplement as recommended by an authority or by the drug/supplement's manufacturer. Capture the recommending authority in the recognizingAuthority property of MedicalEntity.
#[derive()]
pub struct MaximumDoseSchedule {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MaximumDoseSchedule {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// GovernmentBenefitsType enumerates several kinds of government benefits to support the COVID-19 situation. Note that this structure may not capture all benefits offered.
#[derive()]
pub struct GovernmentBenefitsType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GovernmentBenefitsType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A map.
#[derive()]
pub struct Map {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Map {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A sequential publication of comic stories under a
///     	unifying title, for example "The Amazing Spider-Man" or "Groo the
///     	Wanderer".
#[derive()]
pub struct ComicSeries {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ComicSeries {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A code for a medical entity.
#[derive()]
pub struct MedicalCode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalCode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A hotel room is a single room in a hotel.
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
/// 
#[derive()]
pub struct HotelRoom {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HotelRoom {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An instance of a [[Course]] which is distinct from other instances because it is offered at a different time or location or through different media or modes of study or to a specific section of students.
#[derive()]
pub struct CourseInstance {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CourseInstance {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Food event.
#[derive()]
pub struct FoodEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FoodEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A series of movies. Included movies can be indicated with the hasPart property.
#[derive()]
pub struct MovieSeries {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MovieSeries {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A financial product for the loaning of an amount of money, or line of credit, under agreed terms and charges.
#[derive()]
pub struct LoanOrCredit {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LoanOrCredit {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A CreativeWorkSeries in schema.org is a group of related items, typically but not necessarily of the same kind. CreativeWorkSeries are usually organized into some order, often chronological. Unlike [[ItemList]] which is a general purpose data structure for lists of things, the emphasis with CreativeWorkSeries is on published materials (written e.g. books and periodicals, or media such as TV, radio and games).\n\nSpecific subtypes are available for describing [[TVSeries]], [[RadioSeries]], [[MovieSeries]], [[BookSeries]], [[Periodical]] and [[VideoGameSeries]]. In each case, the [[hasPart]] / [[isPartOf]] properties can be used to relate the CreativeWorkSeries to its parts. The general CreativeWorkSeries type serves largely just to organize these more specific and practical subtypes.\n\nIt is common for properties applicable to an item from the series to be usefully applied to the containing group. Schema.org attempts to anticipate some of these cases, but publishers should be free to apply properties of the series parts to the series as a whole wherever they seem appropriate.
/// 	  
#[derive()]
pub struct CreativeWorkSeries {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CreativeWorkSeries {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Report generated by governmental or non-governmental organization.
#[derive()]
pub struct Report {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Report {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A software application designed specifically to work well on a mobile device such as a telephone.
#[derive()]
pub struct MobileApplication {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MobileApplication {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[Claim]] in Schema.org represents a specific, factually-oriented claim that could be the [[itemReviewed]] in a [[ClaimReview]]. The content of a claim can be summarized with the [[text]] property. Variations on well known claims can have their common identity indicated via [[sameAs]] links, and summarized with a [[name]]. Ideally, a [[Claim]] description includes enough contextual information to minimize the risk of ambiguity or inclarity. In practice, many claims are better understood in the context in which they appear or the interpretations provided by claim reviews.
/// 
///   Beyond [[ClaimReview]], the Claim type can be associated with related creative works - for example a [[ScholarlyArticle]] or [[Question]] might be [[about]] some [[Claim]].
/// 
///   At this time, Schema.org does not define any types of relationship between claims. This is a natural area for future exploration.
///   
#[derive()]
pub struct Claim {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Claim {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The most generic type of entity related to health and the practice of medicine.
#[derive()]
pub struct MedicalEntity {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalEntity {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of installing an application.
#[derive()]
pub struct InstallAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InstallAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A summary of how users have interacted with this CreativeWork. In most cases, authors will use a subtype to specify the specific type of interaction.
#[derive()]
pub struct InteractionCounter {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InteractionCounter {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of producing a painting, typically with paint and canvas as instruments.
#[derive()]
pub struct PaintAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PaintAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Individual comic issues are serially published as
///     	part of a larger series. For the sake of consistency, even one-shot issues
///     	belong to a series comprised of a single issue. All comic issues can be
///     	uniquely identified by: the combination of the name and volume number of the
///     	series to which the issue belongs; the issue number; and the variant
///     	description of the issue (if any).
#[derive()]
pub struct ComicIssue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ComicIssue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of gaining ownership of an object from an origin. Reciprocal of GiveAction.\n\nRelated actions:\n\n* [[GiveAction]]: The reciprocal of TakeAction.\n* [[ReceiveAction]]: Unlike ReceiveAction, TakeAction implies that ownership has been transferred.
#[derive()]
pub struct TakeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TakeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Organization: A business corporation.
#[derive()]
pub struct Corporation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Corporation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of adding at a specific location in an ordered collection.
#[derive()]
pub struct InsertAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InsertAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates different price types, for example list price, invoice price, and sale price.
#[derive()]
pub struct PriceTypeEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PriceTypeEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An [[OfferForPurchase]] in Schema.org represents an [[Offer]] to sell something, i.e. an [[Offer]] whose
///   [[businessFunction]] is [sell](http://purl.org/goodrelations/v1#Sell.). See [Good Relations](https://en.wikipedia.org/wiki/GoodRelations) for
///   background on the underlying concepts.
///   
#[derive()]
pub struct OfferForPurchase {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OfferForPurchase {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A process of care involving exercise, changes to diet, fitness routines, and other lifestyle changes aimed at improving a health condition.
#[derive()]
pub struct LifestyleModification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LifestyleModification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any bodily activity that enhances or maintains physical fitness and overall health and wellness. Includes activity that is part of daily living and routine, structured exercise, and exercise prescribed as part of a medical treatment or recovery plan.
#[derive()]
pub struct PhysicalActivity {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PhysicalActivity {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates several kinds of product return policies.
#[derive()]
pub struct MerchantReturnEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MerchantReturnEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Aquarium.
#[derive()]
pub struct Aquarium {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Aquarium {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Vital signs are measures of various physiological functions in order to assess the most basic body functions.
#[derive()]
pub struct VitalSign {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VitalSign {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A theater group or company, for example, the Royal Shakespeare Company or Druid Theatre.
#[derive()]
pub struct TheaterGroup {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TheaterGroup {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of obtaining an object under an agreement to return it at a later date. Reciprocal of LendAction.\n\nRelated actions:\n\n* [[LendAction]]: Reciprocal of BorrowAction.
#[derive()]
pub struct BorrowAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BorrowAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of transferring/moving (abstract or concrete) animate or inanimate objects from one place to another.
#[derive()]
pub struct TransferAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TransferAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A scholarly article.
#[derive()]
pub struct ScholarlyArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ScholarlyArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Indicates whether this drug is available by prescription or over-the-counter.
#[derive()]
pub struct DrugPrescriptionStatus {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DrugPrescriptionStatus {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A medical procedure intended primarily for palliative purposes, aimed at relieving the symptoms of an underlying health condition.
#[derive()]
pub struct PalliativeProcedure {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PalliativeProcedure {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A roofing contractor.
#[derive()]
pub struct RoofingContractor {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RoofingContractor {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A public swimming pool.
#[derive()]
pub struct PublicSwimmingPool {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PublicSwimmingPool {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A cafe or coffee shop.
#[derive()]
pub struct CafeOrCoffeeShop {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CafeOrCoffeeShop {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A structured value providing information about when a certain organization or person owned a certain product.
#[derive()]
pub struct OwnershipInfo {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OwnershipInfo {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any specific branch of medical science or practice. Medical specialities include clinical specialties that pertain to particular organ systems and their respective disease states, as well as allied health specialties. Enumerated type.
#[derive()]
pub struct MedicalSpecialty {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalSpecialty {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A file containing slides or used for a presentation.
#[derive()]
pub struct PresentationDigitalDocument {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PresentationDigitalDocument {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A post office.
#[derive()]
pub struct PostOffice {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PostOffice {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A spreadsheet file.
#[derive()]
pub struct SpreadsheetDigitalDocument {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SpreadsheetDigitalDocument {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// WebContent is a type representing all [[WebPage]], [[WebSite]] and [[WebPageElement]] content. It is sometimes the case that detailed distinctions between Web pages, sites and their parts are not always important or obvious. The  [[WebContent]] type makes it easier to describe Web-addressable content without requiring such distinctions to always be stated. (The intent is that the existing types [[WebPage]], [[WebSite]] and [[WebPageElement]] will eventually be declared as subtypes of [[WebContent]].)
#[derive()]
pub struct WebContent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WebContent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A music venue.
#[derive()]
pub struct MusicVenue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicVenue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A restaurant.
#[derive()]
pub struct Restaurant {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Restaurant {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The LearningResource type can be used to indicate [[CreativeWork]]s (whether physical or digital) that have a particular and explicit orientation towards learning, education, skill acquisition, and other educational purposes.
/// 
/// [[LearningResource]] is expected to be used as an addition to a primary type such as [[Book]], [[VideoObject]], [[Product]] etc.
/// 
/// [[EducationEvent]] serves a similar purpose for event-like things (e.g. a [[Trip]]). A [[LearningResource]] may be created as a result of an [[EducationEvent]], for example by recording one.
#[derive()]
pub struct LearningResource {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LearningResource {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// [[HealthTopicContent]] is [[WebContent]] that is about some aspect of a health topic, e.g. a condition, its symptoms or treatments. Such content may be comprised of several parts or sections and use different types of media. Multiple instances of [[WebContent]] (and hence [[HealthTopicContent]]) can be related using [[hasPart]] / [[isPartOf]] where there is some kind of content hierarchy, and their content described with [[about]] and [[mentions]] e.g. building upon the existing [[MedicalCondition]] vocabulary.
///   
#[derive()]
pub struct HealthTopicContent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HealthTopicContent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A video file.
#[derive()]
pub struct VideoObject {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VideoObject {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A software application.
#[derive()]
pub struct SoftwareApplication {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SoftwareApplication {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of giving money in return for temporary use, but not ownership, of an object such as a vehicle or property. For example, an agent rents a property from a landlord in exchange for a periodic payment.
#[derive()]
pub struct RentAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RentAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A gym.
#[derive()]
pub struct ExerciseGym {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ExerciseGym {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A class of medical drugs, e.g., statins. Classes can represent general pharmacological class, common mechanisms of action, common physiological effects, etc.
#[derive()]
pub struct DrugClass {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DrugClass {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A book, document, or piece of music written by hand rather than typed or printed.
#[derive()]
pub struct Manuscript {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Manuscript {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A piece of sculpture.
#[derive()]
pub struct Sculpture {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Sculpture {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An order is a confirmation of a transaction (a receipt), which can contain multiple line items, each represented by an Offer that has been accepted by the customer.
#[derive()]
pub struct Order {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Order {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A florist.
#[derive()]
pub struct Florist {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Florist {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A shop that will buy, or lend money against the security of, personal possessions.
#[derive()]
pub struct PawnShop {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PawnShop {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A list of items of any sort&#x2014;for example, Top 10 Movies About Weathermen, or Top 100 Party Songs. Not to be confused with HTML lists, which are often used only for formatting.
#[derive()]
pub struct ItemList {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ItemList {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Professional service: Attorney. \n\nThis type is deprecated - [[LegalService]] is more inclusive and less ambiguous.
#[derive()]
pub struct Attorney {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Attorney {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A real-estate agent.
#[derive()]
pub struct RealEstateAgent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RealEstateAgent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A subclass of Role used to describe roles within organizations.
#[derive()]
pub struct OrganizationRole {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OrganizationRole {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A range of services that will be provided to a customer free of charge in case of a defect or malfunction of a product.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#Labor-BringIn\n* http://purl.org/goodrelations/v1#PartsAndLabor-BringIn\n* http://purl.org/goodrelations/v1#PartsAndLabor-PickUp
///       
#[derive()]
pub struct WarrantyScope {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WarrantyScope {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Lists or enumerations—for example, a list of cuisines or music genres, etc.
#[derive()]
pub struct Enumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Enumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Season dedicated to radio broadcast and associated online delivery.
#[derive()]
pub struct RadioSeason {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RadioSeason {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A datasheet or vendor specification of a product (in the sense of a prototypical description).
#[derive()]
pub struct ProductModel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ProductModel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A winery.
#[derive()]
pub struct Winery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Winery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A canal, like the Panama Canal.
#[derive()]
pub struct Canal {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Canal {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A men's clothing store.
#[derive()]
pub struct MensClothingStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MensClothingStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A reservation for boat travel.
/// 
/// Note: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
#[derive()]
pub struct BoatReservation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BoatReservation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A post to a social media platform, including blog posts, tweets, Facebook posts, etc.
#[derive()]
pub struct SocialMediaPosting {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SocialMediaPosting {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A large, usually printed placard, bill, or announcement, often illustrated, that is posted to advertise or publicize something.
#[derive()]
pub struct Poster {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Poster {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of physically/electronically taking delivery of an object that has been transferred from an origin to a destination. Reciprocal of SendAction.\n\nRelated actions:\n\n* [[SendAction]]: The reciprocal of ReceiveAction.\n* [[TakeAction]]: Unlike TakeAction, ReceiveAction does not imply that the ownership has been transferred (e.g. I can receive a package, but it does not mean the package is now mine).
#[derive()]
pub struct ReceiveAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReceiveAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A utility class that serves as the umbrella for a number of 'intangible' things such as quantities, structured values, etc.
#[derive()]
pub struct Intangible {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Intangible {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A recycling center.
#[derive()]
pub struct RecyclingCenter {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RecyclingCenter {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Comedy event.
#[derive()]
pub struct ComedyEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ComedyEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A particular physical or virtual business of an organization for medical purposes. Examples of MedicalBusiness include different businesses run by health professionals.
#[derive()]
pub struct MedicalBusiness {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalBusiness {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A train station.
#[derive()]
pub struct TrainStation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TrainStation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An event involving the delivery of an item.
#[derive()]
pub struct DeliveryEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DeliveryEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A museum.
#[derive()]
pub struct Museum {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Museum {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any medical test, typically performed for diagnostic purposes.
#[derive()]
pub struct MedicalTest {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalTest {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A part of a successively published publication such as a periodical or multi-volume work, often numbered. It may represent a time span, such as a year.\n\nSee also [blog post](http://blog.schema.org/2014/09/schemaorg-support-for-bibliographic_2.html).
#[derive()]
pub struct PublicationVolume {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PublicationVolume {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of finding an object.\n\nRelated actions:\n\n* [[SearchAction]]: FindAction is generally lead by a SearchAction, but not necessarily.
#[derive()]
pub struct FindAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FindAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A waterfall, like Niagara.
#[derive()]
pub struct Waterfall {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Waterfall {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of playing/exercising/training/performing for enjoyment, leisure, recreation, competition or exercise.\n\nRelated actions:\n\n* [[ListenAction]]: Unlike ListenAction (which is under ConsumeAction), PlayAction refers to performing for an audience or at an event, rather than consuming music.\n* [[WatchAction]]: Unlike WatchAction (which is under ConsumeAction), PlayAction refers to showing/displaying for an audience or at an event, rather than consuming visual content.
#[derive()]
pub struct PlayAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PlayAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A series of books. Included books can be indicated with the hasPart property.
#[derive()]
pub struct BookSeries {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BookSeries {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Web page type: Collection page.
#[derive()]
pub struct CollectionPage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CollectionPage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A particular online business, either standalone or the online part of a broader organization. Examples include an eCommerce site, an online travel booking site, an online learning site, an online logistics and shipping provider, an online (virtual) doctor, etc.
#[derive()]
pub struct OnlineBusiness {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OnlineBusiness {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A trip on a commercial train line.
#[derive()]
pub struct TrainTrip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TrainTrip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A painting.
#[derive()]
pub struct Painting {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Painting {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A tourist attraction.  In principle any Thing can be a [[TouristAttraction]], from a [[Mountain]] and [[LandmarksOrHistoricalBuildings]] to a [[LocalBusiness]].  This Type can be used on its own to describe a general [[TouristAttraction]], or be used as an [[additionalType]] to add tourist attraction properties to any other type.  (See examples below)
#[derive()]
pub struct TouristAttraction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TouristAttraction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Represents additional information about a relationship or property. For example a Role can be used to say that a 'member' role linking some SportsTeam to a player occurred during a particular time period. Or that a Person's 'actor' role in a Movie was for some particular characterName. Such properties can be attached to a Role entity, which is then associated with the main entities using ordinary properties like 'member' or 'actor'.\n\nSee also [blog post](http://blog.schema.org/2014/06/introducing-role.html).
#[derive()]
pub struct Role {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Role {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A FundingAgency is an organization that implements one or more [[FundingScheme]]s and manages
///     the granting process (via [[Grant]]s, typically [[MonetaryGrant]]s).
///     A funding agency is not always required for grant funding, e.g. philanthropic giving, corporate sponsorship etc.
///     
/// Examples of funding agencies include ERC, REA, NIH, Bill and Melinda Gates Foundation, ...
///     
#[derive()]
pub struct FundingAgency {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FundingAgency {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A country.
#[derive()]
pub struct Country {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Country {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Season dedicated to TV broadcast and associated online delivery.
#[derive()]
pub struct TVSeason {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TVSeason {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A simple system that adds up the number of risk factors to yield a score that is associated with prognosis, e.g. CHAD score, TIMI risk score.
#[derive()]
pub struct MedicalRiskScore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalRiskScore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The price asked for a given offer by the respective organization or person.
#[derive()]
pub struct UnitPriceSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UnitPriceSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A treatment of people with physical, emotional, or social problems, using purposeful activity to help them overcome or learn to deal with their problems.
#[derive()]
pub struct OccupationalTherapy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OccupationalTherapy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An airport.
#[derive()]
pub struct Airport {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Airport {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of expressing a preference from a set of options or a large or unbounded set of choices/options.
#[derive()]
pub struct ChooseAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ChooseAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Describes a reservation for travel, dining or an event. Some reservations require tickets. \n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, restaurant reservations, flights, or rental cars, use [[Offer]].
#[derive()]
pub struct Reservation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Reservation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A video game series.
#[derive()]
pub struct VideoGameSeries {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VideoGameSeries {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of editing a recipient by removing one of its objects.
#[derive()]
pub struct DeleteAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DeleteAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The status of a medical study. Enumerated type.
#[derive()]
pub struct MedicalStudyStatus {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalStudyStatus {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent approves/certifies/likes/supports/sanctions an object.
#[derive()]
pub struct EndorseAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EndorseAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A movie.
#[derive()]
pub struct Movie {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Movie {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates different price components that together make up the total price for an offered product.
#[derive()]
pub struct PriceComponentTypeEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PriceComponentTypeEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An outlet store.
#[derive()]
pub struct OutletStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OutletStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of  departing from a place. An agent departs from a fromLocation for a destination, optionally with participants.
#[derive()]
pub struct DepartAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DepartAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A home goods store.
#[derive()]
pub struct HomeGoodsStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HomeGoodsStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A web page. Every web page is implicitly assumed to be declared to be of type WebPage, so the various properties about that webpage, such as <code>breadcrumb</code> may be used. We recommend explicit declaration if these properties are specified, but if they are found outside of an itemscope, they will be assumed to be about the page.
#[derive()]
pub struct WebPage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WebPage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A collection of music tracks.
#[derive()]
pub struct MusicAlbum {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicAlbum {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A pond.
#[derive()]
pub struct Pond {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Pond {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of achieving victory in a competitive activity.
#[derive()]
pub struct WinAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WinAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Information about the engine of the vehicle. A vehicle can have multiple engines represented by multiple engine specification entities.
#[derive()]
pub struct EngineSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EngineSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An OfferCatalog is an ItemList that contains related Offers and/or further OfferCatalogs that are offeredBy the same provider.
#[derive()]
pub struct OfferCatalog {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OfferCatalog {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An event venue.
#[derive()]
pub struct EventVenue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EventVenue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Place of worship, such as a church, synagogue, or mosque.
#[derive()]
pub struct PlaceOfWorship {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PlaceOfWorship {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A type of boarding policy used by an airline.
#[derive()]
pub struct BoardingPolicyType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BoardingPolicyType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A seasonal override of a return policy, for example used for holidays.
#[derive()]
pub struct MerchantReturnPolicySeasonalOverride {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MerchantReturnPolicySeasonalOverride {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[NewsArticle]] providing historical context, definition and detail on a specific topic (aka "explainer" or "backgrounder"). For example, an in-depth article or frequently-asked-questions ([FAQ](https://en.wikipedia.org/wiki/FAQ)) document on topics such as Climate Change or the European Union. Other kinds of background material from a non-news setting are often described using [[Book]] or [[Article]], in particular [[ScholarlyArticle]]. See also [[NewsArticle]] for related vocabulary from a learning/education perspective.
#[derive()]
pub struct BackgroundNewsArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BackgroundNewsArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A product taken by mouth that contains a dietary ingredient intended to supplement the diet. Dietary ingredients may include vitamins, minerals, herbs or other botanicals, amino acids, and substances such as enzymes, organ tissues, glandulars and metabolites.
#[derive()]
pub struct DietarySupplement {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DietarySupplement {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any offered product or service. For example: a pair of shoes; a concert ticket; the rental of a car; a haircut; or an episode of a TV show streamed online.
#[derive()]
pub struct Product {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Product {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A delivery service through which content is provided via broadcast over the air or online.
#[derive()]
pub struct BroadcastService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BroadcastService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A FloorPlan is an explicit representation of a collection of similar accommodations, allowing the provision of common information (room counts, sizes, layout diagrams) and offers for rental or sale. In typical use, some [[ApartmentComplex]] has an [[accommodationFloorPlan]] which is a [[FloorPlan]].  A FloorPlan is always in the context of a particular place, either a larger [[ApartmentComplex]] or a single [[Apartment]]. The visual/spatial aspects of a floor plan (i.e. room layout, [see wikipedia](https://en.wikipedia.org/wiki/Floor_plan)) can be indicated using [[image]]. 
#[derive()]
pub struct FloorPlan {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FloorPlan {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An account that allows an investor to deposit funds and place investment orders with a licensed broker or brokerage firm.
#[derive()]
pub struct BrokerageAccount {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BrokerageAccount {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A supply consumed when performing the instructions for how to achieve a result.
#[derive()]
pub struct HowToSupply {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HowToSupply {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A chemical or biologic substance, used as a medical therapy, that has a physiological effect on an organism. Here the term drug is used interchangeably with the term medicine although clinical knowledge makes a clear difference between them.
#[derive()]
pub struct Drug {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Drug {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A stadium.
#[derive()]
pub struct StadiumOrArena {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for StadiumOrArena {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A publication event, e.g. catch-up TV or radio podcast, during which a program is available on-demand.
#[derive()]
pub struct OnDemandEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OnDemandEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of manipulating/administering/supervising/controlling one or more objects.
#[derive()]
pub struct OrganizeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OrganizeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A set of Category Code values.
#[derive()]
pub struct CategoryCodeSet {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CategoryCodeSet {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A music store.
#[derive()]
pub struct MusicStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A PerformanceRole is a Role that some entity places with regard to a theatrical performance, e.g. in a Movie, TVSeries etc.
#[derive()]
pub struct PerformanceRole {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PerformanceRole {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any matter of defined composition that has discrete existence, whose origin may be biological, mineral or chemical.
#[derive()]
pub struct Substance {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Substance {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Instances of the class [[Observation]] are used to specify observations about an entity at a particular time. The principal properties of an [[Observation]] are [[observationAbout]], [[measuredProperty]], [[statType]], [[value] and [[observationDate]]  and [[measuredProperty]]. Some but not all Observations represent a [[QuantitativeValue]]. Quantitative observations can be about a [[StatisticalVariable]], which is an abstract specification about which we can make observations that are grounded at a particular location and time. 
///     
/// Observations can also encode a subset of simple RDF-like statements (its observationAbout, a StatisticalVariable, defining the measuredPoperty; its observationAbout property indicating the entity the statement is about, and [[value]] )
/// 
/// In the context of a quantitative knowledge graph, typical properties could include [[measuredProperty]], [[observationAbout]], [[observationDate]], [[value]], [[unitCode]], [[unitText]], [[measurementMethod]].
///     
#[derive()]
pub struct Observation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Observation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A reservation for an event like a concert, sporting event, or lecture.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
#[derive()]
pub struct EventReservation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EventReservation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A lake (for example, Lake Pontrachain).
#[derive()]
pub struct LakeBodyOfWater {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LakeBodyOfWater {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Exhibition event, e.g. at a museum, library, archive, tradeshow, ...
#[derive()]
pub struct ExhibitionEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ExhibitionEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[RealEstateListing]] is a listing that describes one or more real-estate [[Offer]]s (whose [[businessFunction]] is typically to lease out, or to sell).
///   The [[RealEstateListing]] type itself represents the overall listing, as manifested in some [[WebPage]].
///   
#[derive()]
pub struct RealEstateListing {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RealEstateListing {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumeration of considerations that make a product relevant or potentially restricted for adults only.
#[derive()]
pub struct AdultOrientedEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AdultOrientedEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An order item is a line of an order. It includes the quantity and shipping details of a bought offer.
#[derive()]
pub struct OrderItem {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OrderItem {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Web page type: Video gallery page.
#[derive()]
pub struct VideoGallery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VideoGallery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A WebSite is a set of related web pages and other items typically served from a single web domain and accessible via URLs.
#[derive()]
pub struct WebSite {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WebSite {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of allocating an action/event/task to some destination (someone or something).
#[derive()]
pub struct AssignAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AssignAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A DatedMoneySpecification represents monetary values with optional start and end dates. For example, this could represent an employee's salary over a specific period of time. __Note:__ This type has been superseded by [[MonetaryAmount]], use of that type is recommended.
#[derive()]
pub struct DatedMoneySpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DatedMoneySpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A media season, e.g. TV, radio, video game etc.
#[derive()]
pub struct CreativeWorkSeason {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CreativeWorkSeason {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of swallowing liquids.
#[derive()]
pub struct DrinkAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DrinkAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Web page type: Profile page.
#[derive()]
pub struct ProfilePage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ProfilePage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of transferring ownership of an object to a destination. Reciprocal of TakeAction.\n\nRelated actions:\n\n* [[TakeAction]]: Reciprocal of GiveAction.\n* [[SendAction]]: Unlike SendAction, GiveAction implies that ownership is being transferred (e.g. I may send my laptop to you, but that doesn't mean I'm giving it to you).
#[derive()]
pub struct GiveAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GiveAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Pathogenic bacteria that cause bacterial infection.
#[derive()]
pub struct Bacteria {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Bacteria {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A sea (for example, the Caspian sea).
#[derive()]
pub struct SeaBodyOfWater {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SeaBodyOfWater {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of forming a personal connection with someone/something (object) unidirectionally/asymmetrically to get updates polled from.\n\nRelated actions:\n\n* [[BefriendAction]]: Unlike BefriendAction, FollowAction implies that the connection is *not* necessarily reciprocal.\n* [[SubscribeAction]]: Unlike SubscribeAction, FollowAction implies that the follower acts as an active agent constantly/actively polling for updates.\n* [[RegisterAction]]: Unlike RegisterAction, FollowAction implies that the agent is interested in continuing receiving updates from the object.\n* [[JoinAction]]: Unlike JoinAction, FollowAction implies that the agent is interested in getting updates from the object.\n* [[TrackAction]]: Unlike TrackAction, FollowAction refers to the polling of updates of all aspects of animate objects rather than the location of inanimate objects (e.g. you track a package, but you don't follow it).
#[derive()]
pub struct FollowAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FollowAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A mountain, like Mount Whitney or Mount Everest.
#[derive()]
pub struct Mountain {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Mountain {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A golf course.
#[derive()]
pub struct GolfCourse {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GolfCourse {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A sports club.
#[derive()]
pub struct SportsClub {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SportsClub {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A set of characteristics belonging to businesses, e.g. who compose an item's target audience.
#[derive()]
pub struct BusinessAudience {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BusinessAudience {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An embassy.
#[derive()]
pub struct Embassy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Embassy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A patient-reported or observed dosing schedule for a drug or supplement.
#[derive()]
pub struct ReportedDoseSchedule {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReportedDoseSchedule {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A set of characteristics describing parents, who can be interested in viewing some content.
#[derive()]
pub struct ParentAudience {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ParentAudience {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A book.
#[derive()]
pub struct Book {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Book {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A publication in any medium issued in successive parts bearing numerical or chronological designations and intended to continue indefinitely, such as a magazine, scholarly journal, or newspaper.\n\nSee also [blog post](http://blog.schema.org/2014/09/schemaorg-support-for-bibliographic_2.html).
#[derive()]
pub struct Periodical {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Periodical {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Residence type: Gated community.
#[derive()]
pub struct GatedResidenceCommunity {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GatedResidenceCommunity {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The ConstraintNode type is provided to support usecases in which a node in a structured data graph is described with properties which appear to describe a single entity, but are being used in a situation where they serve a more abstract purpose. A [[ConstraintNode]] can be described using [[constraintProperty]] and [[numConstraints]]. These constraint properties can serve a 
///     variety of purposes, and their values may sometimes be understood to indicate sets of possible values rather than single, exact and specific values.
#[derive()]
pub struct ConstraintNode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ConstraintNode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A specific question - e.g. from a user seeking answers online, or collected in a Frequently Asked Questions (FAQ) document.
#[derive()]
pub struct Question {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Question {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of inserting at the end if an ordered collection.
#[derive()]
pub struct AppendAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AppendAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [hackathon](https://en.wikipedia.org/wiki/Hackathon) event.
#[derive()]
pub struct Hackathon {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Hackathon {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The [[ReportageNewsArticle]] type is a subtype of [[NewsArticle]] representing
///  news articles which are the result of journalistic news reporting conventions.
/// 
/// In practice many news publishers produce a wide variety of article types, many of which might be considered a [[NewsArticle]] but not a [[ReportageNewsArticle]]. For example, opinion pieces, reviews, analysis, sponsored or satirical articles, or articles that combine several of these elements.
/// 
/// The [[ReportageNewsArticle]] type is based on a stricter ideal for "news" as a work of journalism, with articles based on factual information either observed or verified by the author, or reported and verified from knowledgeable sources.  This often includes perspectives from multiple viewpoints on a particular issue (distinguishing news reports from public relations or propaganda).  News reports in the [[ReportageNewsArticle]] sense de-emphasize the opinion of the author, with commentary and value judgements typically expressed elsewhere.
/// 
/// A [[ReportageNewsArticle]] which goes deeper into analysis can also be marked with an additional type of [[AnalysisNewsArticle]].
/// 
#[derive()]
pub struct ReportageNewsArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReportageNewsArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Bed and breakfast.
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
/// 
#[derive()]
pub struct BedAndBreakfast {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BedAndBreakfast {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates several kinds of policies for product return fees.
#[derive()]
pub struct ReturnFeesEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReturnFeesEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of an agent relocating to a place.\n\nRelated actions:\n\n* [[TransferAction]]: Unlike TransferAction, the subject of the move is a living Person or Organization rather than an inanimate object.
#[derive()]
pub struct MoveAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MoveAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any medical imaging modality typically used for diagnostic purposes.
#[derive()]
pub struct ImagingTest {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ImagingTest {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A structured value representing repayment.
#[derive()]
pub struct RepaymentSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RepaymentSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A 3D model represents some kind of 3D content, which may have [[encoding]]s in one or more [[MediaObject]]s. Many 3D formats are available (e.g. see [Wikipedia](https://en.wikipedia.org/wiki/Category:3D_graphics_file_formats)); specific encoding formats can be represented using the [[encodingFormat]] property applied to the relevant [[MediaObject]]. For the
/// case of a single file published after Zip compression, the convention of appending '+zip' to the [[encodingFormat]] can be used. Geospatial, AR/VR, artistic/animation, gaming, engineering and scientific content can all be represented using [[3DModel]].
#[derive()]
pub struct ThreeDModel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ThreeDModel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A school.
#[derive()]
pub struct School {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for School {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Short story or tale. A brief work of literature, usually written in narrative prose.
#[derive()]
pub struct ShortStory {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ShortStory {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// ATM/cash machine.
#[derive()]
pub struct AutomatedTeller {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AutomatedTeller {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A clothing store.
#[derive()]
pub struct ClothingStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ClothingStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A car wash business.
#[derive()]
pub struct AutoWash {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AutoWash {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A facility, often associated with a hospital or medical school, that is devoted to the specific diagnosis and/or healthcare. Previously limited to outpatients but with evolution it may be open to inpatients as well.
#[derive()]
pub struct MedicalClinic {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalClinic {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A medical procedure intended primarily for therapeutic purposes, aimed at improving a health condition.
#[derive()]
pub struct TherapeuticProcedure {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TherapeuticProcedure {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A list of possible levels for the legal validity of a legislation.
#[derive()]
pub struct LegalValueLevel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LegalValueLevel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A US-style health insurance plan network. 
#[derive()]
pub struct HealthPlanNetwork {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HealthPlanNetwork {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of swallowing solid objects.
#[derive()]
pub struct EatAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EatAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A statement about something, for example a fun or interesting fact. If known, the main entity this statement is about can be indicated using mainEntity. For more formal claims (e.g. in Fact Checking), consider using [[Claim]] instead. Use the [[text]] property to capture the text of the statement.
#[derive()]
pub struct Statement {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Statement {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates the EU energy efficiency classes A-G as well as A+, A++, and A+++ as defined in EU directive 2017/1369.
#[derive()]
pub struct EUEnergyEfficiencyEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EUEnergyEfficiencyEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates common size systems for different categories of products, for example "EN-13402" or "UK" for wearables or "Imperial" for screws.
#[derive()]
pub struct SizeSystemEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SizeSystemEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of responding instinctively and emotionally to an object, expressing a sentiment.
#[derive()]
pub struct ReactAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReactAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Indicates employment-related experience requirements, e.g. [[monthsOfExperience]].
#[derive()]
pub struct OccupationalExperienceRequirements {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OccupationalExperienceRequirements {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The average rating based on multiple ratings or reviews.
#[derive()]
pub struct AggregateRating {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AggregateRating {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A collection of datasets.
#[derive()]
pub struct DataCatalog {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DataCatalog {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any constitutionally or isotopically distinct atom, molecule, ion, ion pair, radical, radical ion, complex, conformer etc., identifiable as a separately distinguishable entity.
#[derive()]
pub struct MolecularEntity {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MolecularEntity {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[CompleteDataFeed]] is a [[DataFeed]] whose standard representation includes content for every item currently in the feed.
/// 
/// This is the equivalent of Atom's element as defined in Feed Paging and Archiving [RFC 5005](https://tools.ietf.org/html/rfc5005), for example (and as defined for Atom), when using data from a feed that represents a collection of items that varies over time (e.g. "Top Twenty Records") there is no need to have newer entries mixed in alongside older, obsolete entries. By marking this feed as a CompleteDataFeed, old entries can be safely discarded when the feed is refreshed, since we can assume the feed has provided descriptions for all current items.
#[derive()]
pub struct CompleteDataFeed {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CompleteDataFeed {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A dry-cleaning business.
#[derive()]
pub struct DryCleaningOrLaundry {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DryCleaningOrLaundry {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An [[OfferForLease]] in Schema.org represents an [[Offer]] to lease out something, i.e. an [[Offer]] whose
///   [[businessFunction]] is [lease out](http://purl.org/goodrelations/v1#LeaseOut.). See [Good Relations](https://en.wikipedia.org/wiki/GoodRelations) for
///   background on the underlying concepts.
///   
#[derive()]
pub struct OfferForLease {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OfferForLease {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// [[StatisticalVariable]] represents any type of statistical metric that can be measured at a place and time. The usage pattern for [[StatisticalVariable]] is typically expressed using [[Observation]] with an explicit [[populationType]], which is a type, typically drawn from Schema.org. Each [[StatisticalVariable]] is marked as a [[ConstraintNode]], meaning that some properties (those listed using [[constraintProperty]]) serve in this setting solely to define the statistical variable rather than literally describe a specific person, place or thing. For example, a [[StatisticalVariable]] Median_Height_Person_Female representing the median height of women, could be written as follows: the population type is [[Person]]; the measuredProperty [[height]]; the [[statType]] [[median]]; the [[gender]] [[Female]]. It is important to note that there are many kinds of scientific quantitative observation which are not fully, perfectly or unambiguously described following this pattern, or with solely Schema.org terminology. The approach taken here is designed to allow partial, incremental or minimal description of [[StatisticalVariable]]s, and the use of detailed sets of entity and property IDs from external repositories. The [[measurementMethod]], [[unitCode]] and [[unitText]] properties can also be used to clarify the specific nature and notation of an observed measurement. 
#[derive()]
pub struct StatisticalVariable {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for StatisticalVariable {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A rating is an evaluation on a numeric scale, such as 1 to 5 stars.
#[derive()]
pub struct Rating {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Rating {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A police station.
#[derive()]
pub struct PoliceStation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PoliceStation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A list of possible product availability options.
#[derive()]
pub struct ItemAvailability {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ItemAvailability {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A syllabus that describes the material covered in a course, often with several such sections per [[Course]] so that a distinct [[timeRequired]] can be provided for that section of the [[Course]].
#[derive()]
pub struct Syllabus {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Syllabus {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A nail salon.
#[derive()]
pub struct NailSalon {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for NailSalon {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Animal shelter.
#[derive()]
pub struct AnimalShelter {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AnimalShelter {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
#[derive()]
pub struct UserPageVisits {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserPageVisits {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A statistical distribution of values.
#[derive()]
pub struct QuantitativeValueDistribution {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for QuantitativeValueDistribution {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Level of evidence for a medical guideline. Enumerated type.
#[derive()]
pub struct MedicalEvidenceLevel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalEvidenceLevel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of arriving at a place. An agent arrives at a destination from a fromLocation, optionally with participants.
#[derive()]
pub struct ArriveAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ArriveAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A short band of tough, flexible, fibrous connective tissue that functions to connect multiple bones, cartilages, and structurally support joints.
#[derive()]
pub struct Ligament {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Ligament {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A product provided to consumers and businesses by financial institutions such as banks, insurance companies, brokerage firms, consumer finance companies, and investment companies which comprise the financial services industry.
#[derive()]
pub struct FinancialProduct {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FinancialProduct {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Education event.
#[derive()]
pub struct EducationEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EducationEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A single, identifiable product instance (e.g. a laptop with a particular serial number).
#[derive()]
pub struct IndividualProduct {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for IndividualProduct {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An ice cream shop.
#[derive()]
pub struct IceCreamShop {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for IceCreamShop {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A video game is an electronic game that involves human interaction with a user interface to generate visual feedback on a video device.
#[derive()]
pub struct VideoGame {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VideoGame {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A sub-grouping of food or drink items in a menu. E.g. courses (such as 'Dinner', 'Breakfast', etc.), specific type of dishes (such as 'Meat', 'Vegan', 'Drinks', etc.), or some other classification made by the menu provider.
#[derive()]
pub struct MenuSection {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MenuSection {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Single-celled organism that causes an infection.
#[derive()]
pub struct Protozoa {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Protozoa {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of taking money from a buyer in exchange for goods or services rendered. An agent sells an object, product, or service to a buyer for a price. Reciprocal of BuyAction.
#[derive()]
pub struct SellAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SellAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any rule set or interactive tool for estimating the risk of developing a complication or condition.
#[derive()]
pub struct MedicalRiskEstimator {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalRiskEstimator {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Categories that represent an assessment of the risk of fetal injury due to a drug or pharmaceutical used as directed by the mother during pregnancy.
#[derive()]
pub struct DrugPregnancyCategory {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DrugPregnancyCategory {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A FundingScheme combines organizational, project and policy aspects of grant-based funding
///     that sets guidelines, principles and mechanisms to support other kinds of projects and activities.
///     Funding is typically organized via [[Grant]] funding. Examples of funding schemes: Swiss Priority Programmes (SPPs); EU Framework 7 (FP7); Horizon 2020; the NIH-R01 Grant Program; Wellcome institutional strategic support fund. For large scale public sector funding, the management and administration of grant awards is often handled by other, dedicated, organizations - [[FundingAgency]]s such as ERC, REA, ...
#[derive()]
pub struct FundingScheme {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FundingScheme {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An ocean (for example, the Pacific).
#[derive()]
pub struct OceanBodyOfWater {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OceanBodyOfWater {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A BreadcrumbList is an ItemList consisting of a chain of linked Web pages, typically described using at least their URL and their name, and typically ending with the current page.\n\nThe [[position]] property is used to reconstruct the order of the items in a BreadcrumbList. The convention is that a breadcrumb list has an [[itemListOrder]] of [[ItemListOrderAscending]] (lower values listed first), and that the first items in this list correspond to the "top" or beginning of the breadcrumb trail, e.g. with a site or section homepage. The specific values of 'position' are not assigned meaning for a BreadcrumbList, but they should be integers, e.g. beginning with '1' for the first item in the list.
///       
#[derive()]
pub struct BreadcrumbList {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BreadcrumbList {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A public structure, such as a town hall or concert hall.
#[derive()]
pub struct CivicStructure {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CivicStructure {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An office equipment store.
#[derive()]
pub struct OfficeEquipmentStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OfficeEquipmentStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Health and beauty.
#[derive()]
pub struct HealthAndBeautyBusiness {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HealthAndBeautyBusiness {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
#[derive()]
pub struct UserCheckins {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserCheckins {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Printed music, as opposed to performed or recorded music.
#[derive()]
pub struct SheetMusic {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SheetMusic {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of expressing a positive sentiment about the object. An agent likes an object (a proposition, topic or theme) with participants.
#[derive()]
pub struct LikeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LikeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Research Organization (e.g. scientific institute, research company).
#[derive()]
pub struct ResearchOrganization {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ResearchOrganization {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A product or service offered by a bank whereby one may deposit, withdraw or transfer money and in some cases be paid interest.
#[derive()]
pub struct BankAccount {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BankAccount {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A part of a successively published publication such as a periodical or publication volume, often numbered, usually containing a grouping of works such as articles.\n\nSee also [blog post](http://blog.schema.org/2014/09/schemaorg-support-for-bibliographic_2.html).
#[derive()]
pub struct PublicationIssue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PublicationIssue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A loan in which property or real estate is used as collateral. (A loan securitized against some real estate.)
#[derive()]
pub struct MortgageLoan {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MortgageLoan {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An enumeration of genders.
#[derive()]
pub struct GenderType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GenderType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An anatomical system is a group of anatomical structures that work together to perform a certain task. Anatomical systems, such as organ systems, are one organizing principle of anatomy, and can include circulatory, digestive, endocrine, integumentary, immune, lymphatic, muscular, nervous, reproductive, respiratory, skeletal, urinary, vestibular, and other systems.
#[derive()]
pub struct AnatomicalSystem {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AnatomicalSystem {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A unique instance of a radio BroadcastService on a CableOrSatelliteService lineup.
#[derive()]
pub struct RadioChannel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RadioChannel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A state or province of a country.
#[derive()]
pub struct State {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for State {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Organization: Political Party.
#[derive()]
pub struct PoliticalParty {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PoliticalParty {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An observational study is a type of medical study that attempts to infer the possible effect of a treatment through observation of a cohort of subjects over a period of time. In an observational study, the assignment of subjects into treatment groups versus control groups is outside the control of the investigator. This is in contrast with controlled studies, such as the randomized controlled trials represented by MedicalTrial, where each subject is randomly assigned to a treatment group or a control group before the start of the treatment.
#[derive()]
pub struct MedicalObservationalStudy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalObservationalStudy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A radio station.
#[derive()]
pub struct RadioStation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RadioStation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A place offering space for "Recreational Vehicles", Caravans, mobile homes and the like.
#[derive()]
pub struct RVPark {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RVPark {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A reservation for bus travel. \n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
#[derive()]
pub struct BusReservation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BusReservation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any recommendation made by a standard society (e.g. ACC/AHA) or consensus statement that denotes how to diagnose and treat a particular condition. Note: this type should be used to tag the actual guideline recommendation; if the guideline recommendation occurs in a larger scholarly article, use MedicalScholarlyArticle to tag the overall article, not this type. Note also: the organization making the recommendation should be captured in the recognizingAuthority base property of MedicalEntity.
#[derive()]
pub struct MedicalGuideline {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalGuideline {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A convenience store.
#[derive()]
pub struct ConvenienceStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ConvenienceStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A file containing a note, primarily for the author.
#[derive()]
pub struct NoteDigitalDocument {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for NoteDigitalDocument {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Specifies a location feature by providing a structured value representing a feature of an accommodation as a property-value pair of varying degrees of formality.
#[derive()]
pub struct LocationFeatureSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LocationFeatureSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A recommended dosing schedule for a drug or supplement as prescribed or recommended by an authority or by the drug/supplement's manufacturer. Capture the recommending authority in the recognizingAuthority property of MedicalEntity.
#[derive()]
pub struct RecommendedDoseSchedule {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RecommendedDoseSchedule {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of un-registering from a service.\n\nRelated actions:\n\n* [[RegisterAction]]: antonym of UnRegisterAction.\n* [[LeaveAction]]: Unlike LeaveAction, UnRegisterAction implies that you are unregistering from a service you were previously registered, rather than leaving a team/group of people.
#[derive()]
pub struct UnRegisterAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UnRegisterAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A shop that sells alcoholic drinks such as wine, beer, whisky and other spirits.
#[derive()]
pub struct LiquorStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LiquorStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A scholarly article in the medical domain.
#[derive()]
pub struct MedicalScholarlyArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalScholarlyArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A text file. The text can be unformatted or contain markup, html, etc.
#[derive()]
pub struct TextObject {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TextObject {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Target audiences types for medical web pages. Enumerated type.
#[derive()]
pub struct MedicalAudienceType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalAudienceType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A food or drink item listed in a menu or menu section.
#[derive()]
pub struct MenuItem {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MenuItem {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A set of defined terms, for example a set of categories or a classification scheme, a glossary, dictionary or enumeration.
#[derive()]
pub struct DefinedTermSet {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DefinedTermSet {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A hostel - cheap accommodation, often in shared dormitories.
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
/// 
#[derive()]
pub struct Hostel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Hostel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A monetary value or range. This type can be used to describe an amount of money such as $50 USD, or a range as in describing a bank account being suitable for a balance between £1,000 and £1,000,000 GBP, or the value of a salary, etc. It is recommended to use [[PriceSpecification]] Types to describe the price of an Offer, Invoice, etc.
#[derive()]
pub struct MonetaryAmount {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MonetaryAmount {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An entry point, within some Web-based protocol.
#[derive()]
pub struct EntryPoint {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EntryPoint {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A podcast is an episodic series of digital audio or video files which a user can download and listen to.
#[derive()]
pub struct PodcastSeries {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PodcastSeries {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A quotation. Often but not necessarily from some written work, attributable to a real world author and - if associated with a fictional character - to any fictional Person. Use [[isBasedOn]] to link to source/origin. The [[recordedIn]] property can be used to reference a Quotation from an [[Event]].
#[derive()]
pub struct Quotation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Quotation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A means for accessing a service, e.g. a government office location, web site, or phone number.
#[derive()]
pub struct ServiceChannel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ServiceChannel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An organization with archival holdings. An organization which keeps and preserves archival material and typically makes it accessible to the public.
#[derive()]
pub struct ArchiveOrganization {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ArchiveOrganization {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [blog](https://en.wikipedia.org/wiki/Blog), sometimes known as a "weblog". Note that the individual posts ([[BlogPosting]]s) in a [[Blog]] are often colloquially referred to by the same term.
#[derive()]
pub struct Blog {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Blog {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// RsvpResponseType is an enumeration type whose instances represent responding to an RSVP request.
#[derive()]
pub struct RsvpResponseType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RsvpResponseType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent leaves an event / group with participants/friends at a location.\n\nRelated actions:\n\n* [[JoinAction]]: The antonym of LeaveAction.\n* [[UnRegisterAction]]: Unlike UnRegisterAction, LeaveAction implies leaving a group/team of people rather than a service.
#[derive()]
pub struct LeaveAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LeaveAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A complex mathematical calculation requiring an online calculator, used to assess prognosis. Note: use the url property of Thing to record any URLs for online calculators.
#[derive()]
pub struct MedicalRiskCalculator {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalRiskCalculator {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A statistical distribution of monetary amounts.
#[derive()]
pub struct MonetaryAmountDistribution {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MonetaryAmountDistribution {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A government office&#x2014;for example, an IRS or DMV office.
#[derive()]
pub struct GovernmentOffice {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GovernmentOffice {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Rigid connective tissue that comprises up the skeletal structure of the human body.
#[derive()]
pub struct Bone {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Bone {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An image of a visual machine-readable code such as a barcode or QR code.
#[derive()]
pub struct Barcode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Barcode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A description of an educational course which may be offered as distinct instances which take place at different times or take place at different locations, or be offered through different media or modes of study. An educational course is a sequence of one or more educational events and/or creative works which aims to build knowledge, competence or ability of learners.
#[derive()]
pub struct Course {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Course {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Computer programming source code. Example: Full (compile ready) solutions, code snippet samples, scripts, templates.
#[derive()]
pub struct SoftwareSourceCode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SoftwareSourceCode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An indication for preventing an underlying condition, symptom, etc.
#[derive()]
pub struct PreventionIndication {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PreventionIndication {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of forming a personal connection with someone/something (object) unidirectionally/asymmetrically to get updates pushed to.\n\nRelated actions:\n\n* [[FollowAction]]: Unlike FollowAction, SubscribeAction implies that the subscriber acts as a passive agent being constantly/actively pushed for updates.\n* [[RegisterAction]]: Unlike RegisterAction, SubscribeAction implies that the agent is interested in continuing receiving updates from the object.\n* [[JoinAction]]: Unlike JoinAction, SubscribeAction implies that the agent is interested in continuing receiving updates from the object.
#[derive()]
pub struct SubscribeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SubscribeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of being defeated in a competitive activity.
#[derive()]
pub struct LoseAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LoseAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates several types of product return methods.
#[derive()]
pub struct ReturnMethodEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReturnMethodEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A volcano, like Fujisan.
#[derive()]
pub struct Volcano {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Volcano {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A preschool.
#[derive()]
pub struct Preschool {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Preschool {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An airline flight.
#[derive()]
pub struct Flight {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Flight {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent orders an object/product/service to be delivered/sent.
#[derive()]
pub struct OrderAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OrderAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A sub-grouping of steps in the instructions for how to achieve a result (e.g. steps for making a pie crust within a pie recipe).
#[derive()]
pub struct HowToSection {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HowToSection {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Search and Rescue organization of some kind.
#[derive()]
pub struct SearchRescueOrganization {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SearchRescueOrganization {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of physically/electronically dispatching an object for transfer from an origin to a destination. Related actions:\n\n* [[ReceiveAction]]: The reciprocal of SendAction.\n* [[GiveAction]]: Unlike GiveAction, SendAction does not imply the transfer of ownership (e.g. I can send you my laptop, but I'm not necessarily giving it to you).
#[derive()]
pub struct SendAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SendAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// This is the [[Action]] of navigating to a specific [[startOffset]] timestamp within a [[VideoObject]], typically represented with a URL template structure.
#[derive()]
pub struct SeekToAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SeekToAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A set of organisms asserted to represent a natural cohesive biological unit.
#[derive()]
pub struct Taxon {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Taxon {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A subscription which allows a user to access media including audio, video, books, etc.
#[derive()]
pub struct MediaSubscription {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MediaSubscription {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The geographic shape of a place. A GeoShape can be described using several properties whose values are based on latitude/longitude pairs. Either whitespace or commas can be used to separate latitude and longitude; whitespace should be used when writing a list of several such points.
#[derive()]
pub struct GeoShape {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GeoShape {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An explanation in the instructions for how to achieve a result. It provides supplementary information about a technique, supply, author's preference, etc. It can explain what could be done, or what should not be done, but doesn't specify what should be done (see HowToDirection).
#[derive()]
pub struct HowToTip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HowToTip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An electronic file or document.
#[derive()]
pub struct DigitalDocument {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DigitalDocument {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An alternative, closely-related condition typically considered later in the differential diagnosis process along with the signs that are used to distinguish it.
#[derive()]
pub struct DDxElement {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DDxElement {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A StatisticalPopulation is a set of instances of a certain given type that satisfy some set of constraints. The property [[populationType]] is used to specify the type. Any property that can be used on instances of that type can appear on the statistical population. For example, a [[StatisticalPopulation]] representing all [[Person]]s with a [[homeLocation]] of East Podunk California would be described by applying the appropriate [[homeLocation]] and [[populationType]] properties to a [[StatisticalPopulation]] item that stands for that set of people.
/// The properties [[numConstraints]] and [[constraintProperty]] are used to specify which of the populations properties are used to specify the population. Note that the sense of "population" used here is the general sense of a statistical
/// population, and does not imply that the population consists of people. For example, a [[populationType]] of [[Event]] or [[NewsArticle]] could be used. See also [[Observation]], where a [[populationType]] such as [[Person]] or [[Event]] can be indicated directly. In most cases it may be better to use [[StatisticalVariable]] instead of [[StatisticalPopulation]].
#[derive()]
pub struct StatisticalPopulation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for StatisticalPopulation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A library.
#[derive()]
pub struct Library {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Library {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An advertising section of the page.
#[derive()]
pub struct WPAdBlock {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WPAdBlock {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of searching for an object.\n\nRelated actions:\n\n* [[FindAction]]: SearchAction generally leads to a FindAction, but not necessarily.
#[derive()]
pub struct SearchAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SearchAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates several types of return labels for product returns.
#[derive()]
pub struct ReturnLabelSourceEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReturnLabelSourceEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A playground.
#[derive()]
pub struct Playground {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Playground {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An enumeration that describes different types of medical procedures.
#[derive()]
pub struct MedicalProcedureType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalProcedureType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A specific strength in which a medical drug is available in a specific country.
#[derive()]
pub struct DrugStrength {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DrugStrength {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A US-style health insurance plan, including PPOs, EPOs, and HMOs. 
#[derive()]
pub struct HealthInsurancePlan {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HealthInsurancePlan {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Music event.
#[derive()]
pub struct MusicEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A reservoir of water, typically an artificially created lake, like the Lake Kariba reservoir.
#[derive()]
pub struct Reservoir {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Reservoir {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A collection of items, e.g. creative works or products.
#[derive()]
pub struct Collection {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Collection {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A program offered by an institution which determines the learning progress to achieve an outcome, usually a credential like a degree or certificate. This would define a discrete set of opportunities (e.g., job, courses) that together constitute a program with a clear start, end, set of requirements, and transition to a new occupational opportunity (e.g., a job), or sometimes a higher educational opportunity (e.g., an advanced degree).
#[derive()]
pub struct EducationalOccupationalProgram {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EducationalOccupationalProgram {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A value indicating a special usage of a car, e.g. commercial rental, driving school, or as a taxi.
#[derive()]
pub struct CarUsageType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CarUsageType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Represents an item or group of closely related items treated as a unit for the sake of evaluation in a [[MediaReview]]. Authorship etc. apply to the items rather than to the curation/grouping or reviewing party.
#[derive()]
pub struct MediaReviewItem {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MediaReviewItem {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Quiz: A test of knowledge, skills and abilities.
#[derive()]
pub struct Quiz {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Quiz {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A property-value pair, e.g. representing a feature of a product or place. Use the 'name' property for the name of the property. If there is an additional human-readable version of the value, put that into the 'description' property.\n\n Always use specific schema.org properties when a) they exist and b) you can populate them. Using PropertyValue as a substitute will typically not trigger the same effect as using the original, specific property.
///     
#[derive()]
pub struct PropertyValue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PropertyValue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A reservation for a rental car.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations.
#[derive()]
pub struct RentalCarReservation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RentalCarReservation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A camping site, campsite, or [[Campground]] is a place used for overnight stay in the outdoors, typically containing individual [[CampingPitch]] locations. \n\n
/// In British English a campsite is an area, usually divided into a number of pitches, where people can camp overnight using tents or camper vans or caravans; this British English use of the word is synonymous with the American English expression campground. In American English the term campsite generally means an area where an individual, family, group, or military unit can pitch a tent or park a camper; a campground may contain many campsites (source: Wikipedia, see [https://en.wikipedia.org/wiki/Campsite](https://en.wikipedia.org/wiki/Campsite)).\n\n
/// 
/// See also the dedicated [document on the use of schema.org for marking up hotels and other forms of accommodations](/docs/hotels.html).
/// 
#[derive()]
pub struct Campground {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Campground {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Visual arts event.
#[derive()]
pub struct VisualArtsEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VisualArtsEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An eCommerce site.
#[derive()]
pub struct OnlineStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OnlineStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumeration of common measurement types (or dimensions), for example "chest" for a person, "inseam" for pants, "gauge" for screws, or "wheel" for bicycles.
#[derive()]
pub struct MeasurementTypeEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MeasurementTypeEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Entities that have a somewhat fixed, physical extension.
#[derive()]
pub struct Place {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Place {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A retail good store.
#[derive()]
pub struct Store {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Store {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// One or more messages between organizations or people on a particular topic. Individual messages can be linked to the conversation with isPartOf or hasPart properties.
#[derive()]
pub struct Conversation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Conversation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A resort is a place used for relaxation or recreation, attracting visitors for holidays or vacations. Resorts are places, towns or sometimes commercial establishments operated by a single company (source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/Resort">http://en.wikipedia.org/wiki/Resort</a>).
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
///     
#[derive()]
pub struct Resort {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Resort {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A dance group&#x2014;for example, the Alvin Ailey Dance Theater or Riverdance.
#[derive()]
pub struct DanceGroup {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DanceGroup {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A web page that provides medical information.
#[derive()]
pub struct MedicalWebPage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalWebPage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An over the air or online broadcast event.
#[derive()]
pub struct BroadcastEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BroadcastEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A publication containing information about varied topics that are pertinent to general information, a geographic area, or a specific subject matter (i.e. business, culture, education). Often published daily.
#[derive()]
pub struct Newspaper {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Newspaper {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// USNonprofitType: Non-profit organization type originating from the United States.
#[derive()]
pub struct USNonprofitType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for USNonprofitType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The mailing address.
#[derive()]
pub struct PostalAddress {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PostalAddress {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A payment method is a standardized procedure for transferring the monetary amount for a purchase. Payment methods are characterized by the legal and technical structures used, and by the organization or group carrying out the transaction.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#ByBankTransferInAdvance\n* http://purl.org/goodrelations/v1#ByInvoice\n* http://purl.org/goodrelations/v1#Cash\n* http://purl.org/goodrelations/v1#CheckInAdvance\n* http://purl.org/goodrelations/v1#COD\n* http://purl.org/goodrelations/v1#DirectDebit\n* http://purl.org/goodrelations/v1#GoogleCheckout\n* http://purl.org/goodrelations/v1#PayPal\n* http://purl.org/goodrelations/v1#PaySwarm
///         
#[derive()]
pub struct PaymentMethod {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PaymentMethod {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of asserting that a future event/action is no longer going to happen.\n\nRelated actions:\n\n* [[ConfirmAction]]: The antonym of CancelAction.
#[derive()]
pub struct CancelAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CancelAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent bookmarks/flags/labels/tags/marks an object.
#[derive()]
pub struct BookmarkAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BookmarkAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A movie rental store.
#[derive()]
pub struct MovieRentalStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MovieRentalStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates common types of measurement for wearables products.
#[derive()]
pub struct WearableMeasurementTypeEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WearableMeasurementTypeEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A travel agency.
#[derive()]
pub struct TravelAgency {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TravelAgency {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any branch of a field in which people typically develop specific expertise, usually after significant study, time, and effort.
#[derive()]
pub struct Specialty {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Specialty {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A review created by an end-user (e.g. consumer, purchaser, attendee etc.), in contrast with [[CriticReview]].
#[derive()]
pub struct UserReview {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserReview {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Indicates a range of postal codes, usually defined as the set of valid codes between [[postalCodeBegin]] and [[postalCodeEnd]], inclusively.
#[derive()]
pub struct PostalCodeRangeSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PostalCodeRangeSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A food service, like breakfast, lunch, or dinner.
#[derive()]
pub struct FoodService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FoodService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The status of an Action.
#[derive()]
pub struct ActionStatusType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ActionStatusType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A type of permission which can be granted for accessing a digital document.
#[derive()]
pub struct DigitalDocumentPermissionType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DigitalDocumentPermissionType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An amusement park.
#[derive()]
pub struct AmusementPark {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AmusementPark {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent joins an event/group with participants/friends at a location.\n\nRelated actions:\n\n* [[RegisterAction]]: Unlike RegisterAction, JoinAction refers to joining a group/team of people.\n* [[SubscribeAction]]: Unlike SubscribeAction, JoinAction does not imply that you'll be receiving updates.\n* [[FollowAction]]: Unlike FollowAction, JoinAction does not imply that you'll be polling for updates.
#[derive()]
pub struct JoinAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for JoinAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A grocery store.
#[derive()]
pub struct GroceryStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GroceryStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A reservation for train travel.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
#[derive()]
pub struct TrainReservation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TrainReservation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An indication for a medical therapy that has been formally specified or approved by a regulatory body that regulates use of the therapy; for example, the US FDA approves indications for most drugs in the US.
#[derive()]
pub struct ApprovedIndication {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ApprovedIndication {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A navigation element of the page.
#[derive()]
pub struct SiteNavigationElement {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SiteNavigationElement {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Buddhist temple.
#[derive()]
pub struct BuddhistTemple {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BuddhistTemple {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
#[derive()]
pub struct UserComments {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserComments {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A NewsArticle is an article whose content reports news, or provides background context and supporting materials for understanding the news.
/// 
/// A more detailed overview of [schema.org News markup](/docs/news.html) is also available.
/// 
#[derive()]
pub struct NewsArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for NewsArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of expressing a consistency of opinion with the object. An agent agrees to/about an object (a proposition, topic or theme) with participants.
#[derive()]
pub struct AgreeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AgreeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Financial services business.
#[derive()]
pub struct FinancialService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FinancialService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The Game type represents things which are games. These are typically rule-governed recreational activities, e.g. role-playing games in which players assume the role of characters in a fictional setting.
#[derive()]
pub struct Game {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Game {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A series of [[Event]]s. Included events can relate with the series using the [[superEvent]] property.
/// 
/// An EventSeries is a collection of events that share some unifying characteristic. For example, "The Olympic Games" is a series, which
/// is repeated regularly. The "2012 London Olympics" can be presented both as an [[Event]] in the series "Olympic Games", and as an
/// [[EventSeries]] that included a number of sporting competitions as Events.
/// 
/// The nature of the association between the events in an [[EventSeries]] can vary, but typical examples could
/// include a thematic event series (e.g. topical meetups or classes), or a series of regular events that share a location, attendee group and/or organizers.
/// 
/// EventSeries has been defined as a kind of Event to make it easy for publishers to use it in an Event context without
/// worrying about which kinds of series are really event-like enough to call an Event. In general an EventSeries
/// may seem more Event-like when the period of time is compact and when aspects such as location are fixed, but
/// it may also sometimes prove useful to describe a longer-term series as an Event.
///    
#[derive()]
pub struct EventSeries {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EventSeries {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A predefined value for a product characteristic, e.g. the power cord plug type 'US' or the garment sizes 'S', 'M', 'L', and 'XL'.
#[derive()]
pub struct QualitativeValue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for QualitativeValue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A monetary grant.
#[derive()]
pub struct MonetaryGrant {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MonetaryGrant {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Organization: Sports team.
#[derive()]
pub struct SportsTeam {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SportsTeam {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A statement of the money due for goods or services; a bill.
#[derive()]
pub struct Invoice {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Invoice {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



///  Codes for use with the [[mediaAuthenticityCategory]] property, indicating the authenticity of a media object (in the context of how it was published or shared). In general these codes are not mutually exclusive, although some combinations (such as 'original' versus 'transformed', 'edited' and 'staged') would be contradictory if applied in the same [[MediaReview]]. Note that the application of these codes is with regard to a piece of media shared or published in a particular context.
#[derive()]
pub struct MediaManipulationRatingEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MediaManipulationRatingEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of conveying information to another person via a communication medium (instrument) such as speech, email, or telephone conversation.
#[derive()]
pub struct CommunicateAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CommunicateAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A company or fund that gathers capital from a number of investors to create a pool of money that is then re-invested into stocks, bonds and other assets.
#[derive()]
pub struct InvestmentFund {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InvestmentFund {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent orders a (not yet released) object/product/service to be delivered/sent.
#[derive()]
pub struct PreOrderAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PreOrderAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A fact-checking review of claims made (or reported) in some creative work (referenced via itemReviewed).
#[derive()]
pub struct ClaimReview {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ClaimReview {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A math solver which is capable of solving a subset of mathematical problems.
#[derive()]
pub struct MathSolver {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MathSolver {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of planning the execution of an event/task/action/reservation/plan to a future date.
#[derive()]
pub struct PlanAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PlanAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Used to describe a ticket to an event, a flight, a bus ride, etc.
#[derive()]
pub struct Ticket {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Ticket {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Categories of medical devices, organized by the purpose or intended use of the device.
#[derive()]
pub struct MedicalDevicePurpose {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalDevicePurpose {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A hotel is an establishment that provides lodging paid on a short-term basis (source: Wikipedia, the free encyclopedia, see http://en.wikipedia.org/wiki/Hotel).
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
/// 
#[derive()]
pub struct Hotel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Hotel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A church.
#[derive()]
pub struct Church {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Church {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A store that sells materials useful or necessary for various hobbies.
#[derive()]
pub struct HobbyShop {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HobbyShop {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent inspects, determines, investigates, inquires, or examines an object's accuracy, quality, condition, or state.
#[derive()]
pub struct CheckAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CheckAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A bridge.
#[derive()]
pub struct Bridge {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Bridge {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of starting or activating a device or application (e.g. starting a timer or turning on a flashlight).
#[derive()]
pub struct ActivateAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ActivateAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A type of physical examination of a patient performed by a physician. 
#[derive()]
pub struct PhysicalExam {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PhysicalExam {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A blog post.
#[derive()]
pub struct BlogPosting {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BlogPosting {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates several kinds of product return refund types.
#[derive()]
pub struct RefundTypeEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RefundTypeEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An Insurance agency.
#[derive()]
pub struct InsuranceAgency {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InsuranceAgency {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates common size groups (also known as "size types") for wearable products.
#[derive()]
pub struct WearableSizeGroupEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WearableSizeGroupEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Status of a game server.
#[derive()]
pub struct GameServerStatus {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GameServerStatus {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A wholesale store.
#[derive()]
pub struct WholesaleStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WholesaleStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An action performed by a direct agent and indirect participants upon a direct object. Optionally happens at a location with the help of an inanimate instrument. The execution of the action may produce a result. Specific action sub-type documentation specifies the exact expectation of each argument/role.\n\nSee also [blog post](http://blog.schema.org/2014/04/announcing-schemaorg-actions.html) and [Actions overview document](https://schema.org/docs/actions.html).
#[derive()]
pub struct Action {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Action {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An art gallery.
#[derive()]
pub struct ArtGallery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ArtGallery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A risk factor is anything that increases a person's likelihood of developing or contracting a disease, medical condition, or complication.
#[derive()]
pub struct MedicalRiskFactor {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalRiskFactor {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A construction business.\n\nA HomeAndConstructionBusiness is a [[LocalBusiness]] that provides services around homes and buildings.\n\nAs a [[LocalBusiness]] it can be described as a [[provider]] of one or more [[Service]]\(s).
#[derive()]
pub struct HomeAndConstructionBusiness {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HomeAndConstructionBusiness {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of resuming a device or application which was formerly paused (e.g. resume music playback or resume a timer).
#[derive()]
pub struct ResumeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ResumeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A CovidTestingFacility is a [[MedicalClinic]] where testing for the COVID-19 Coronavirus
///       disease is available. If the facility is being made available from an established [[Pharmacy]], [[Hotel]], or other
///       non-medical organization, multiple types can be listed. This makes it easier to re-use existing schema.org information
///       about that place, e.g. contact info, address, opening hours. Note that in an emergency, such information may not always be reliable.
///       
#[derive()]
pub struct CovidTestingFacility {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CovidTestingFacility {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A unique instance of a BroadcastService on a CableOrSatelliteService lineup.
#[derive()]
pub struct BroadcastChannel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BroadcastChannel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of responding to a question/message asked/sent by the object. Related to [[AskAction]].\n\nRelated actions:\n\n* [[AskAction]]: Appears generally as an origin of a ReplyAction.
#[derive()]
pub struct ReplyAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReplyAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of distributing content to people for their amusement or edification.
#[derive()]
pub struct ShareAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ShareAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A strategy of regulating the intake of food to achieve or maintain a specific health-related goal.
#[derive()]
pub struct Diet {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Diet {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A card payment method of a particular brand or name.  Used to mark up a particular payment method and/or the financial product/service that supplies the card account.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#AmericanExpress\n* http://purl.org/goodrelations/v1#DinersClub\n* http://purl.org/goodrelations/v1#Discover\n* http://purl.org/goodrelations/v1#JCB\n* http://purl.org/goodrelations/v1#MasterCard\n* http://purl.org/goodrelations/v1#VISA
///        
#[derive()]
pub struct CreditCard {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CreditCard {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An intangible item that describes an alignment between a learning resource and a node in an educational framework.
/// 
/// Should not be used where the nature of the alignment can be described using a simple property, for example to express that a resource [[teaches]] or [[assesses]] a competency.
#[derive()]
pub struct AlignmentObject {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AlignmentObject {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A locksmith.
#[derive()]
pub struct Locksmith {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Locksmith {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// (Eventually to be defined as) a supertype of GeoShape designed to accommodate definitions from Geo-Spatial best practices.
#[derive()]
pub struct GeospatialGeometry {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GeospatialGeometry {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A jewelry store.
#[derive()]
pub struct JewelryStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for JewelryStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A movie theater.
#[derive()]
pub struct MovieTheater {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MovieTheater {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A short TV program or a segment/part of a TV program.
#[derive()]
pub struct TVClip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TVClip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Systems of medical practice.
#[derive()]
pub struct MedicineSystem {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicineSystem {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[FAQPage]] is a [[WebPage]] presenting one or more "[Frequently asked questions](https://en.wikipedia.org/wiki/FAQ)" (see also [[QAPage]]).
#[derive()]
pub struct FAQPage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FAQPage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A car is a wheeled, self-powered motor vehicle used for transportation.
#[derive()]
pub struct Car {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Car {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Used to describe membership in a loyalty programs (e.g. "StarAliance"), traveler clubs (e.g. "AAA"), purchase clubs ("Safeway Club"), etc.
#[derive()]
pub struct ProgramMembership {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ProgramMembership {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Properties that take Mass as values are of the form '&lt;Number&gt; &lt;Mass unit of measure&gt;'. E.g., '7 kg'.
#[derive()]
pub struct Mass {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Mass {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A structured value representing the duration and scope of services that will be provided to a customer free of charge in case of a defect or malfunction of a product.
#[derive()]
pub struct WarrantyPromise {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WarrantyPromise {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A shoe store.
#[derive()]
pub struct ShoeStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ShoeStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Property value specification.
#[derive()]
pub struct PropertyValueSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PropertyValueSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Properties that take Distances as values are of the form '&lt;Number&gt; &lt;Length unit of measure&gt;'. E.g., '7 ft'.
#[derive()]
pub struct Distance {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Distance {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A service provided by an organization, e.g. delivery service, print services, etc.
#[derive()]
pub struct Service {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Service {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A HyperToc represents a hypertext table of contents for complex media objects, such as [[VideoObject]], [[AudioObject]]. Items in the table of contents are indicated using the [[tocEntry]] property, and typed [[HyperTocEntry]]. For cases where the same larger work is split into multiple files, [[associatedMedia]] can be used on individual [[HyperTocEntry]] items.
#[derive()]
pub struct HyperToc {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HyperToc {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of rejecting to/adopting an object.\n\nRelated actions:\n\n* [[AcceptAction]]: The antonym of RejectAction.
#[derive()]
pub struct RejectAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RejectAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A crematorium.
#[derive()]
pub struct Crematorium {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Crematorium {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A value indicating which roadwheels will receive torque.
#[derive()]
pub struct DriveWheelConfigurationValue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DriveWheelConfigurationValue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A brand is a name used by an organization or business person for labeling a product, product group, or similar.
#[derive()]
pub struct Brand {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Brand {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A QAPage is a WebPage focussed on a specific Question and its Answer(s), e.g. in a question answering site or documenting Frequently Asked Questions (FAQs).
#[derive()]
pub struct QAPage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for QAPage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A process of care relying upon counseling, dialogue and communication  aimed at improving a mental health condition without use of drugs.
#[derive()]
pub struct PsychologicalTreatment {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PsychologicalTreatment {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A store that sells mobile phones and related accessories.
#[derive()]
pub struct MobilePhoneStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MobilePhoneStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A medical test performed on a sample of a patient's blood.
#[derive()]
pub struct BloodTest {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BloodTest {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A tourist information center.
#[derive()]
pub struct TouristInformationCenter {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TouristInformationCenter {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any feature associated or not with a medical condition. In medicine a symptom is generally subjective while a sign is objective.
#[derive()]
pub struct MedicalSignOrSymptom {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalSignOrSymptom {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A legal document such as an act, decree, bill, etc. (enforceable or not) or a component of a legal act (like an article).
#[derive()]
pub struct Legislation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Legislation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of dressing oneself in clothing.
#[derive()]
pub struct WearAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WearAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
#[derive()]
pub struct UserTweets {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserTweets {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A middle school (typically for children aged around 11-14, although this varies somewhat).
#[derive()]
pub struct MiddleSchool {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MiddleSchool {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An answer offered to a question; perhaps correct, perhaps opinionated or wrong.
#[derive()]
pub struct Answer {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Answer {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of consuming static visual content.
#[derive()]
pub struct ViewAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ViewAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A media season, e.g. TV, radio, video game etc.
#[derive()]
pub struct Season {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Season {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A radio channel that uses FM.
#[derive()]
pub struct FMRadioChannel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FMRadioChannel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any part of the human body, typically a component of an anatomical system. Organs, tissues, and cells are all anatomical structures.
#[derive()]
pub struct AnatomicalStructure {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AnatomicalStructure {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An entity holding detailed information about the available bed types, e.g. the quantity of twin beds for a hotel room. For the single case of just one bed of a certain type, you can use bed directly with a text. See also [[BedType]] (under development).
#[derive()]
pub struct BedDetails {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BedDetails {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A DefinedRegion is a geographic area defined by potentially arbitrary (rather than political, administrative or natural geographical) criteria. Properties are provided for defining a region by reference to sets of postal codes.
/// 
/// Examples: a delivery destination when shopping. Region where regional pricing is configured.
/// 
/// Requirement 1:
/// Country: US
/// States: "NY", "CA"
/// 
/// Requirement 2:
/// Country: US
/// PostalCode Set: { [94000-94585], [97000, 97999], [13000, 13599]}
/// { [12345, 12345], [78945, 78945], }
/// Region = state, canton, prefecture, autonomous community...
/// 
#[derive()]
pub struct DefinedRegion {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DefinedRegion {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A tattoo parlor.
#[derive()]
pub struct TattooParlor {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TattooParlor {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Research project.
#[derive()]
pub struct ResearchProject {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ResearchProject {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An image file.
#[derive()]
pub struct ImageObject {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ImageObject {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A theater or other performing art center.
#[derive()]
pub struct PerformingArtsTheater {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PerformingArtsTheater {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A utility class that serves as the umbrella for a number of 'intangible' things in the medical space.
#[derive()]
pub struct MedicalIntangible {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalIntangible {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An apartment (in American English) or flat (in British English) is a self-contained housing unit (a type of residential real estate) that occupies only part of a building (source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/Apartment">http://en.wikipedia.org/wiki/Apartment</a>).
#[derive()]
pub struct Apartment {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Apartment {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A type of blood vessel that specifically carries blood away from the heart.
#[derive()]
pub struct Artery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Artery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A reservation for a taxi.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations. For offers of tickets, use [[Offer]].
#[derive()]
pub struct TaxiReservation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TaxiReservation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Intended audience for an item, i.e. the group for whom the item was created.
#[derive()]
pub struct Audience {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Audience {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Reference documentation for application programming interfaces (APIs).
#[derive()]
pub struct APIReference {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for APIReference {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A dentist.
#[derive()]
pub struct Dentist {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Dentist {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A condition or factor that indicates use of a medical therapy, including signs, symptoms, risk factors, anatomical states, etc.
#[derive()]
pub struct MedicalIndication {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalIndication {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A page devoted to a single item, such as a particular product or hotel.
#[derive()]
pub struct ItemPage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ItemPage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent tracks an object for updates.\n\nRelated actions:\n\n* [[FollowAction]]: Unlike FollowAction, TrackAction refers to the interest on the location of innanimates objects.\n* [[SubscribeAction]]: Unlike SubscribeAction, TrackAction refers to  the interest on the location of innanimate objects.
#[derive()]
pub struct TrackAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TrackAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A bowling alley.
#[derive()]
pub struct BowlingAlley {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BowlingAlley {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A river (for example, the broad majestic Shannon).
#[derive()]
pub struct RiverBodyOfWater {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RiverBodyOfWater {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A specific and exact (byte-for-byte) version of an [[AudioObject]]. Two byte-for-byte identical files, for the purposes of this type, considered identical. If they have different embedded metadata the files will differ. Different external facts about the files, e.g. creator or dateCreated that aren't represented in their actual content, do not affect this notion of identity.
#[derive()]
pub struct AudioObjectSnapshot {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AudioObjectSnapshot {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An infectious disease is a clinically evident human disease resulting from the presence of pathogenic microbial agents, like pathogenic viruses, pathogenic bacteria, fungi, protozoa, multicellular parasites, and prions. To be considered an infectious disease, such pathogens are known to be able to cause this disease.
#[derive()]
pub struct InfectiousDisease {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InfectiousDisease {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The day of the week, e.g. used to specify to which day the opening hours of an OpeningHoursSpecification refer.
/// 
/// Originally, URLs from [GoodRelations](http://purl.org/goodrelations/v1) were used (for [[Monday]], [[Tuesday]], [[Wednesday]], [[Thursday]], [[Friday]], [[Saturday]], [[Sunday]] plus a special entry for [[PublicHolidays]]); these have now been integrated directly into schema.org.
///       
#[derive()]
pub struct DayOfWeek {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DayOfWeek {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An email message.
#[derive()]
pub struct EmailMessage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EmailMessage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A medical trial is a type of medical study that uses a scientific process to compare the safety and efficacy of medical therapies or medical procedures. In general, medical trials are controlled and subjects are allocated at random to the different treatment and/or control groups.
#[derive()]
pub struct MedicalTrial {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalTrial {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of expressing a difference of opinion with the object. An agent disagrees to/about an object (a proposition, topic or theme) with participants.
#[derive()]
pub struct DisagreeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DisagreeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A set of characteristics belonging to people, e.g. who compose an item's target audience.
#[derive()]
pub struct PeopleAudience {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PeopleAudience {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A review of an item - for example, of a restaurant, movie, or store.
#[derive()]
pub struct Review {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Review {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[comment]] that corrects [[CreativeWork]].
#[derive()]
pub struct CorrectionComment {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CorrectionComment {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A pet store.
#[derive()]
pub struct PetStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PetStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of registering to an organization/service without the guarantee to receive it.\n\nRelated actions:\n\n* [[RegisterAction]]: Unlike RegisterAction, ApplyAction has no guarantees that the application will be accepted.
#[derive()]
pub struct ApplyAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ApplyAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A city hall.
#[derive()]
pub struct CityHall {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CityHall {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerated for values for itemListOrder for indicating how an ordered ItemList is organized.
#[derive()]
pub struct ItemListOrderType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ItemListOrderType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
#[derive()]
pub struct UserInteraction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserInteraction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A posting to a discussion forum.
#[derive()]
pub struct DiscussionForumPosting {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DiscussionForumPosting {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A sports location, such as a playing field.
#[derive()]
pub struct SportsActivityLocation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SportsActivityLocation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Series in schema.org is a group of related items, typically but not necessarily of the same kind. See also [[CreativeWorkSeries]], [[EventSeries]].
#[derive()]
pub struct Series {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Series {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An list item, e.g. a step in a checklist or how-to description.
#[derive()]
pub struct ListItem {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ListItem {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An enumeration of several kinds of Map.
#[derive()]
pub struct MapCategoryType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MapCategoryType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Structured values are used when the value of a property has a more complex structure than simply being a textual value or a reference to another thing.
#[derive()]
pub struct StructuredValue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for StructuredValue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A contact point&#x2014;for example, a Customer Complaints department.
#[derive()]
pub struct ContactPoint {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ContactPoint {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A comedy club.
#[derive()]
pub struct ComedyClub {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ComedyClub {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Brewery.
#[derive()]
pub struct Brewery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Brewery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The artwork on the cover of a comic.
#[derive()]
pub struct ComicCoverArt {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ComicCoverArt {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A type of blood vessel that specifically carries blood to the heart.
#[derive()]
pub struct Vein {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Vein {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
#[derive()]
pub struct UserPlusOnes {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserPlusOnes {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A lodging business, such as a motel, hotel, or inn.
#[derive()]
pub struct LodgingBusiness {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LodgingBusiness {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[CriticReview]] is a more specialized form of Review written or published by a source that is recognized for its reviewing activities. These can include online columns, travel and food guides, TV and radio shows, blogs and other independent Web sites. [[CriticReview]]s are typically more in-depth and professionally written. For simpler, casually written user/visitor/viewer/customer reviews, it is more appropriate to use the [[UserReview]] type. Review aggregator sites such as Metacritic already separate out the site's user reviews from selected critic reviews that originate from third-party sources.
#[derive()]
pub struct CriticReview {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CriticReview {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent pays a price to a participant.
#[derive()]
pub struct PayAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PayAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The action that takes in a math expression and directs users to a page potentially capable of solving/simplifying that expression.
#[derive()]
pub struct SolveMathAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SolveMathAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of interacting with another person or organization.
#[derive()]
pub struct InteractAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InteractAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Size related properties of a product, typically a size code ([[name]]) and optionally a [[sizeSystem]], [[sizeGroup]], and product measurements ([[hasMeasurement]]). In addition, the intended audience can be defined through [[suggestedAge]], [[suggestedGender]], and suggested body measurements ([[suggestedMeasurement]]).
#[derive()]
pub struct SizeSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SizeSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A business providing entertainment.
#[derive()]
pub struct EntertainmentBusiness {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EntertainmentBusiness {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A reservation to dine at a food-related business.\n\nNote: This type is for information about actual reservations, e.g. in confirmation emails or HTML pages with individual confirmations of reservations.
#[derive()]
pub struct FoodEstablishmentReservation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FoodEstablishmentReservation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A sidebar section of the page.
#[derive()]
pub struct WPSideBar {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WPSideBar {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The business function specifies the type of activity or access (i.e., the bundle of rights) offered by the organization or business person through the offer. Typical are sell, rental or lease, maintenance or repair, manufacture / produce, recycle / dispose, engineering / construction, or installation. Proprietary specifications of access rights are also instances of this class.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#ConstructionInstallation\n* http://purl.org/goodrelations/v1#Dispose\n* http://purl.org/goodrelations/v1#LeaseOut\n* http://purl.org/goodrelations/v1#Maintain\n* http://purl.org/goodrelations/v1#ProvideService\n* http://purl.org/goodrelations/v1#Repair\n* http://purl.org/goodrelations/v1#Sell\n* http://purl.org/goodrelations/v1#Buy
///         
#[derive()]
pub struct BusinessFunction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BusinessFunction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A media object, such as an image, video, audio, or text object embedded in a web page or a downloadable dataset i.e. DataDownload. Note that a creative work may have many media objects associated with it on the same web page. For example, a page about a single song (MusicRecording) may have a music video (VideoObject), and a high and low bandwidth audio stream (2 AudioObject's).
#[derive()]
pub struct MediaObject {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MediaObject {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A body of structured information describing some topic(s) of interest.
#[derive()]
pub struct Dataset {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Dataset {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The delivery of a parcel either via the postal service or a commercial service.
#[derive()]
pub struct ParcelDelivery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ParcelDelivery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A list of possible statuses for the legal force of a legislation.
#[derive()]
pub struct LegalForceStatus {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LegalForceStatus {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Web applications.
#[derive()]
pub struct WebApplication {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WebApplication {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A bus station.
#[derive()]
pub struct BusStation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BusStation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of managing by changing/editing the state of the object.
#[derive()]
pub struct UpdateAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UpdateAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A music video file.
#[derive()]
pub struct MusicVideoObject {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicVideoObject {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A compound price specification is one that bundles multiple prices that all apply in combination for different dimensions of consumption. Use the name property of the attached unit price specification for indicating the dimension of a price component (e.g. "electricity" or "final cleaning").
#[derive()]
pub struct CompoundPriceSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CompoundPriceSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// CreativeWorkSeries dedicated to TV broadcast and associated online delivery.
#[derive()]
pub struct TVSeries {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TVSeries {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A common pathway for the electrochemical nerve impulses that are transmitted along each of the axons.
#[derive()]
pub struct Nerve {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Nerve {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Researchers.
#[derive()]
pub struct Researcher {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Researcher {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A picture or diagram made with a pencil, pen, or crayon rather than paint.
#[derive()]
pub struct Drawing {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Drawing {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A short segment/part of a movie.
#[derive()]
pub struct MovieClip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MovieClip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any anatomical structure which pertains to the soft nervous tissue functioning as the coordinating center of sensation and intellectual and nervous activity.
#[derive()]
pub struct BrainStructure {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BrainStructure {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A house painting service.
#[derive()]
pub struct HousePainter {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HousePainter {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A motorized bicycle is a bicycle with an attached motor used to power the vehicle, or to assist with pedaling.
#[derive()]
pub struct MotorizedBicycle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MotorizedBicycle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of capturing sound and moving images on film, video, or digitally.
#[derive()]
pub struct FilmAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FilmAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A tire shop.
#[derive()]
pub struct TireShop {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TireShop {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The costs of settling the payment using a particular payment method.
#[derive()]
pub struct PaymentChargeSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PaymentChargeSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// For a given health insurance plan, the specification for costs and coverage of prescription drugs. 
#[derive()]
pub struct HealthPlanFormulary {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HealthPlanFormulary {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of intentionally disregarding the object. An agent ignores an object.
#[derive()]
pub struct IgnoreAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for IgnoreAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UKNonprofitType: Non-profit organization type originating from the United Kingdom.
#[derive()]
pub struct UKNonprofitType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UKNonprofitType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A car rental business.
#[derive()]
pub struct AutoRental {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AutoRental {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A trip on a commercial bus line.
#[derive()]
pub struct BusTrip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BusTrip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A play is a form of literature, usually consisting of dialogue between characters, intended for theatrical performance rather than just reading. Note: A performance of a Play would be a [[TheaterEvent]] or [[BroadcastEvent]] - the *Play* being the [[workPerformed]].
#[derive()]
pub struct Play {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Play {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A specific dosing schedule for a drug or supplement.
#[derive()]
pub struct DoseSchedule {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DoseSchedule {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A single item within a larger data feed.
#[derive()]
pub struct DataFeedItem {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DataFeedItem {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An [[Article]] whose content is primarily [[satirical]](https://en.wikipedia.org/wiki/Satire) in nature, i.e. unlikely to be literally true. A satirical article is sometimes but not necessarily also a [[NewsArticle]]. [[ScholarlyArticle]]s are also sometimes satirized.
#[derive()]
pub struct SatiricalArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SatiricalArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Pathogenic virus that causes viral infection.
#[derive()]
pub struct Virus {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Virus {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of consuming audio content.
#[derive()]
pub struct ListenAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ListenAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A SpeakableSpecification indicates (typically via [[xpath]] or [[cssSelector]]) sections of a document that are highlighted as particularly [[speakable]]. Instances of this type are expected to be used primarily as values of the [[speakable]] property.
#[derive()]
pub struct SpeakableSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SpeakableSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of generating a comment about a subject.
#[derive()]
pub struct CommentAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CommentAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An accommodation is a place that can accommodate human beings, e.g. a hotel room, a camping pitch, or a meeting room. Many accommodations are for overnight stays, but this is not a mandatory requirement.
/// For more specific types of accommodations not defined in schema.org, one can use [[additionalType]] with external vocabularies.
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
/// 
#[derive()]
pub struct Accommodation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Accommodation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



///  A point value or interval for product characteristics and other purposes.
#[derive()]
pub struct QuantitativeValue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for QuantitativeValue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: A social dance.
#[derive()]
pub struct DanceEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DanceEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of organizing tasks/objects/events by associating resources to it.
#[derive()]
pub struct AllocateAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AllocateAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Beauty salon.
#[derive()]
pub struct BeautySalon {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BeautySalon {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A hair salon.
#[derive()]
pub struct HairSalon {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HairSalon {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Accountancy business.\n\nAs a [[LocalBusiness]] it can be described as a [[provider]] of one or more [[Service]]\(s).
///       
#[derive()]
pub struct AccountingService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AccountingService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A synagogue.
#[derive()]
pub struct Synagogue {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Synagogue {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A fire station. With firemen.
#[derive()]
pub struct FireStation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FireStation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
#[derive()]
pub struct UserLikes {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserLikes {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A type of bed. This is used for indicating the bed or beds available in an accommodation.
#[derive()]
pub struct BedType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BedType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A radio channel that uses AM.
#[derive()]
pub struct AMRadioChannel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AMRadioChannel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A listing that describes a job opening in a certain organization.
#[derive()]
pub struct JobPosting {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for JobPosting {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of providing goods, services, or money without compensation, often for philanthropic reasons.
#[derive()]
pub struct DonateAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DonateAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of notifying someone that a future event/action is going to happen as expected.\n\nRelated actions:\n\n* [[CancelAction]]: The antonym of ConfirmAction.
#[derive()]
pub struct ConfirmAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ConfirmAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// UserInteraction and its subtypes is an old way of talking about users interacting with pages. It is generally better to use [[Action]]-based vocabulary, alongside types such as [[Comment]].
#[derive()]
pub struct UserDownloads {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UserDownloads {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A process of progressive physical care and rehabilitation aimed at improving a health condition.
#[derive()]
pub struct PhysicalTherapy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PhysicalTherapy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// When a single product is associated with multiple offers (for example, the same pair of shoes is offered by different merchants), then AggregateOffer can be used.\n\nNote: AggregateOffers are normally expected to associate multiple offers that all share the same defined [[businessFunction]] value, or default to http://purl.org/goodrelations/v1#Sell if businessFunction is not explicitly defined.
#[derive()]
pub struct AggregateOffer {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AggregateOffer {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A set of requirements that must be fulfilled in order to perform an Action.
#[derive()]
pub struct ActionAccessSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ActionAccessSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An enterprise (potentially individual but typically collaborative), planned to achieve a particular aim.
/// Use properties from [[Organization]], [[subOrganization]]/[[parentOrganization]] to indicate project sub-structures. 
///    
#[derive()]
pub struct Project {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Project {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of downloading an object.
#[derive()]
pub struct DownloadAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DownloadAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Used to indicate whether a product is EnergyStar certified.
#[derive()]
pub struct EnergyStarEnergyEfficiencyEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EnergyStarEnergyEfficiencyEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Instructions that explain how to achieve a result by performing a sequence of steps.
#[derive()]
pub struct HowTo {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HowTo {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Organization: Non-governmental Organization.
#[derive()]
pub struct NGO {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for NGO {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A meeting room, conference room, or conference hall is a room provided for singular events such as business conferences and meetings (source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/Conference_hall">http://en.wikipedia.org/wiki/Conference_hall</a>).
/// <br /><br />
/// See also the <a href="/docs/hotels.html">dedicated document on the use of schema.org for marking up hotels and other forms of accommodations</a>.
/// 
#[derive()]
pub struct MeetingRoom {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MeetingRoom {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Festival.
#[derive()]
pub struct Festival {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Festival {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A vehicle is a device that is designed or used to transport people or cargo over land, water, air, or through space.
#[derive()]
pub struct Vehicle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Vehicle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of notifying an event organizer as to whether you expect to attend the event.
#[derive()]
pub struct RsvpAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RsvpAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A medical procedure intended primarily for diagnostic, as opposed to therapeutic, purposes.
#[derive()]
pub struct DiagnosticProcedure {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DiagnosticProcedure {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A medical organization (physical or not), such as hospital, institution or clinic.
#[derive()]
pub struct MedicalOrganization {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalOrganization {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A DeliveryTimeSettings represents re-usable pieces of shipping information, relating to timing. It is designed for publication on an URL that may be referenced via the [[shippingSettingsLink]] property of an [[OfferShippingDetails]]. Several occurrences can be published, distinguished (and identified/referenced) by their different values for [[transitTimeLabel]].
#[derive()]
pub struct DeliveryTimeSettings {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DeliveryTimeSettings {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A vet's office.
#[derive()]
pub struct VeterinaryCare {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VeterinaryCare {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of expressing a negative sentiment about the object. An agent dislikes an object (a proposition, topic or theme) with participants.
#[derive()]
pub struct DislikeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DislikeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An EventAttendanceModeEnumeration value is one of potentially several modes of organising an event, relating to whether it is online or offline.
#[derive()]
pub struct EventAttendanceModeEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EventAttendanceModeEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A business that provides Heating, Ventilation and Air Conditioning services.
#[derive()]
pub struct HVACBusiness {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HVACBusiness {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of returning to the origin that which was previously received (concrete objects) or taken (ownership).
#[derive()]
pub struct ReturnAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReturnAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Web page type: About page.
#[derive()]
pub struct AboutPage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AboutPage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A payment method using a credit, debit, store or other card to associate the payment with an account.
#[derive()]
pub struct PaymentCard {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PaymentCard {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Hindu temple.
#[derive()]
pub struct HinduTemple {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HinduTemple {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerates some common technology platforms, for use with properties such as [[actionPlatform]]. It is not supposed to be comprehensive - when a suitable code is not enumerated here, textual or URL values can be used instead. These codes are at a fairly high level and do not deal with versioning and other nuance. Additional codes can be suggested [in github](https://github.com/schemaorg/schemaorg/issues/3057). 
#[derive()]
pub struct DigitalPlatformEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DigitalPlatformEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A radio episode which can be part of a series or season.
#[derive()]
pub struct RadioEpisode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RadioEpisode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A process of care using radiation aimed at improving a health condition.
#[derive()]
pub struct RadiationTherapy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RadiationTherapy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A medical study is an umbrella type covering all kinds of research studies relating to human medicine or health, including observational studies and interventional trials and registries, randomized, controlled or not. When the specific type of study is known, use one of the extensions of this type, such as MedicalTrial or MedicalObservationalStudy. Also, note that this type should be used to mark up data that describes the study itself; to tag an article that publishes the results of a study, use MedicalScholarlyArticle. Note: use the code property of MedicalEntity to store study IDs, e.g. clinicaltrials.gov ID.
#[derive()]
pub struct MedicalStudy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalStudy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A taxi stand.
#[derive()]
pub struct TaxiStand {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TaxiStand {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A single feed providing structured information about one or more entities or topics.
#[derive()]
pub struct DataFeed {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DataFeed {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Design models for medical trials. Enumerated type.
#[derive()]
pub struct MedicalTrialDesign {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalTrialDesign {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of reaching a draw in a competitive activity.
#[derive()]
pub struct TieAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TieAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A public toilet is a room or small building containing one or more toilets (and possibly also urinals) which is available for use by the general public, or by customers or employees of certain businesses.
#[derive()]
pub struct PublicToilet {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PublicToilet {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A motorcycle repair shop.
#[derive()]
pub struct MotorcycleRepair {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MotorcycleRepair {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of editing a recipient by replacing an old object with a new object.
#[derive()]
pub struct ReplaceAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReplaceAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[LibrarySystem]] is a collaborative system amongst several libraries.
#[derive()]
pub struct LibrarySystem {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LibrarySystem {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A specific object or file containing a Legislation. Note that the same Legislation can be published in multiple files. For example, a digitally signed PDF, a plain PDF and an HTML version.
#[derive()]
pub struct LegislationObject {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LegislationObject {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A work of art that is primarily visual in character.
#[derive()]
pub struct VisualArtwork {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VisualArtwork {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A comment on an item - for example, a comment on a blog post. The comment's content is expressed via the [[text]] property, and its topic via [[about]], properties shared with all CreativeWorks.
#[derive()]
pub struct Comment {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Comment {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Consortium is a membership [[Organization]] whose members are typically Organizations.
#[derive()]
pub struct Consortium {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Consortium {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Nutritional information about the recipe.
#[derive()]
pub struct NutritionInformation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for NutritionInformation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A description of costs to the patient under a given network or formulary.
#[derive()]
pub struct HealthPlanCostSharingSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HealthPlanCostSharingSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A Childcare center.
#[derive()]
pub struct ChildCare {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ChildCare {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[MediaReview]] is a more specialized form of Review dedicated to the evaluation of media content online, typically in the context of fact-checking and misinformation.
///     For more general reviews of media in the broader sense, use [[UserReview]], [[CriticReview]] or other [[Review]] types. This definition is
///     a work in progress. While the [[MediaManipulationRatingEnumeration]] list reflects significant community review amongst fact-checkers and others working
///     to combat misinformation, the specific structures for representing media objects, their versions and publication context, are still evolving. Similarly, best practices for the relationship between [[MediaReview]] and [[ClaimReview]] markup have not yet been finalized.
#[derive()]
pub struct MediaReview {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MediaReview {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A media episode (e.g. TV, radio, video game) which can be part of a series or season.
#[derive()]
pub struct Episode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Episode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An educational organization.
#[derive()]
pub struct EducationalOrganization {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for EducationalOrganization {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A PublicationEvent corresponds indifferently to the event of publication for a CreativeWork of any type, e.g. a broadcast event, an on-demand event, a book/journal publication via a variety of delivery media.
#[derive()]
pub struct PublicationEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PublicationEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The legal availability status of a medical drug.
#[derive()]
pub struct DrugLegalStatus {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DrugLegalStatus {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A guideline recommendation that is regarded as efficacious and where quality of the data supporting the recommendation is sound.
#[derive()]
pub struct MedicalGuidelineRecommendation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalGuidelineRecommendation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// For a [[VideoGame]], such as used with a [[PlayGameAction]], an enumeration of the kind of game availability offered. 
#[derive()]
pub struct GameAvailabilityEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GameAvailabilityEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An historical landmark or building.
#[derive()]
pub struct LandmarksOrHistoricalBuildings {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LandmarksOrHistoricalBuildings {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A graveyard.
#[derive()]
pub struct Cemetery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Cemetery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A bar or pub.
#[derive()]
pub struct BarOrPub {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BarOrPub {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Fitness-related activity designed for a specific health-related purpose, including defined exercise routines as well as activity prescribed by a clinician.
#[derive()]
pub struct ExercisePlan {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ExercisePlan {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Residence type: Apartment complex.
#[derive()]
pub struct ApartmentComplex {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ApartmentComplex {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An electrician.
#[derive()]
pub struct Electrician {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Electrician {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A stage of a medical condition, such as 'Stage IIIa'.
#[derive()]
pub struct MedicalConditionStage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalConditionStage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A specific and exact (byte-for-byte) version of an [[ImageObject]]. Two byte-for-byte identical files, for the purposes of this type, considered identical. If they have different embedded metadata (e.g. XMP, EXIF) the files will differ. Different external facts about the files, e.g. creator or dateCreated that aren't represented in their actual content, do not affect this notion of identity.
#[derive()]
pub struct ImageObjectSnapshot {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ImageObjectSnapshot {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A structured value providing information about the opening hours of a place or a certain service inside a place.\n\n
/// The place is __open__ if the [[opens]] property is specified, and __closed__ otherwise.\n\nIf the value for the [[closes]] property is less than the value for the [[opens]] property then the hour range is assumed to span over the next day.
///       
#[derive()]
pub struct OpeningHoursSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OpeningHoursSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Auto body shop.
#[derive()]
pub struct AutoBodyShop {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AutoBodyShop {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The cost per unit of a medical drug. Note that this type is not meant to represent the price in an offer of a drug for sale; see the Offer type for that. This type will typically be used to tag wholesale or average retail cost of a drug, or maximum reimbursable cost. Costs of medical drugs vary widely depending on how and where they are paid for, so while this type captures some of the variables, costs should be used with caution by consumers of this schema's markup.
#[derive()]
pub struct DrugCost {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DrugCost {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A class, also often called a 'Type'; equivalent to rdfs:Class.
#[derive()]
pub struct Class {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Class {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerated options related to a ContactPoint.
#[derive()]
pub struct ContactPointOption {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ContactPointOption {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of editing by adding an object to a collection.
#[derive()]
pub struct AddAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AddAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A component of the human body circulatory system comprised of an intricate network of hollow tubes that transport blood throughout the entire body.
#[derive()]
pub struct Vessel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Vessel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The anatomical location at which two or more bones make contact.
#[derive()]
pub struct Joint {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Joint {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An intangible type to be applied to any archive content, carrying with it a set of properties required to describe archival items and collections.
#[derive()]
pub struct ArchiveComponent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ArchiveComponent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerated status values for Reservation.
#[derive()]
pub struct ReservationStatusType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReservationStatusType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Enumerated status values for Order.
#[derive()]
pub struct OrderStatus {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OrderStatus {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of stopping or deactivating a device or application (e.g. stopping a timer or turning off a flashlight).
#[derive()]
pub struct DeactivateAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DeactivateAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A bookstore.
#[derive()]
pub struct BookStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BookStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Web page type: Image gallery page.
#[derive()]
pub struct ImageGallery {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ImageGallery {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A permit issued by a government agency.
#[derive()]
pub struct GovernmentPermit {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GovernmentPermit {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Indicates whether this game is multi-player, co-op or single-player.
#[derive()]
pub struct GamePlayMode {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GamePlayMode {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of producing a visual/graphical representation of an object, typically with a pen/pencil and paper as instruments.
#[derive()]
pub struct DrawAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DrawAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Web page type: Checkout page.
#[derive()]
pub struct CheckoutPage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CheckoutPage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of momentarily pausing a device or application (e.g. pause music playback or pause a timer).
#[derive()]
pub struct SuspendAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SuspendAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A step in the instructions for how to achieve a result. It is an ordered list with HowToDirection and/or HowToTip items.
#[derive()]
pub struct HowToStep {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HowToStep {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A house is a building or structure that has the ability to be occupied for habitation by humans or other creatures (source: Wikipedia, the free encyclopedia, see <a href="http://en.wikipedia.org/wiki/House">http://en.wikipedia.org/wiki/House</a>).
#[derive()]
pub struct House {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for House {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A School District is an administrative area for the administration of schools.
#[derive()]
pub struct SchoolDistrict {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SchoolDistrict {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A store that sells reading glasses and similar devices for improving vision.
#[derive()]
pub struct Optician {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Optician {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A HyperToEntry is an item within a [[HyperToc]], which represents a hypertext table of contents for complex media objects, such as [[VideoObject]], [[AudioObject]]. The media object itself is indicated using [[associatedMedia]]. Each section of interest within that content can be described with a [[HyperTocEntry]], with associated [[startOffset]] and [[endOffset]]. When several entries are all from the same file, [[associatedMedia]] is used on the overarching [[HyperTocEntry]]; if the content has been split into multiple files, they can be referenced using [[associatedMedia]] on each [[HyperTocEntry]].
#[derive()]
pub struct HyperTocEntry {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HyperTocEntry {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of applying an object to its intended purpose.
#[derive()]
pub struct UseAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for UseAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A trip or journey. An itinerary of visits to one or more places.
#[derive()]
pub struct Trip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Trip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A self-storage facility.
#[derive()]
pub struct SelfStorage {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SelfStorage {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A [[CampingPitch]] is an individual place for overnight stay in the outdoors, typically being part of a larger camping site, or [[Campground]].\n\n
/// In British English a campsite, or campground, is an area, usually divided into a number of pitches, where people can camp overnight using tents or camper vans or caravans; this British English use of the word is synonymous with the American English expression campground. In American English the term campsite generally means an area where an individual, family, group, or military unit can pitch a tent or park a camper; a campground may contain many campsites.
/// (Source: Wikipedia, see [https://en.wikipedia.org/wiki/Campsite](https://en.wikipedia.org/wiki/Campsite).)\n\n
/// See also the dedicated [document on the use of schema.org for marking up hotels and other forms of accommodations](/docs/hotels.html).
/// 
#[derive()]
pub struct CampingPitch {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CampingPitch {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of discovering/finding an object.
#[derive()]
pub struct DiscoverAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DiscoverAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A MerchantReturnPolicy provides information about product return policies associated with an [[Organization]], [[Product]], or [[Offer]].
#[derive()]
pub struct MerchantReturnPolicy {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MerchantReturnPolicy {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An organization that provides flights for passengers.
#[derive()]
pub struct Airline {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Airline {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An agent controls a device or application.
#[derive()]
pub struct ControlAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ControlAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// HealthAspectEnumeration enumerates several aspects of health content online, each of which might be described using [[hasHealthAspect]] and [[HealthTopicContent]].
#[derive()]
pub struct HealthAspectEnumeration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HealthAspectEnumeration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A service to convert funds from one currency to another currency.
#[derive()]
pub struct CurrencyConversionService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for CurrencyConversionService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of forming a personal connection with someone (object) mutually/bidirectionally/symmetrically.\n\nRelated actions:\n\n* [[FollowAction]]: Unlike FollowAction, BefriendAction implies that the connection is reciprocal.
#[derive()]
pub struct BefriendAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BefriendAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of traveling from a fromLocation to a destination by a specified mode of transport, optionally with participants.
#[derive()]
pub struct TravelAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TravelAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A program with both an educational and employment component. Typically based at a workplace and structured around work-based learning, with the aim of instilling competencies related to an occupation. WorkBasedProgram is used to distinguish programs such as apprenticeships from school, college or other classroom based educational programs.
#[derive()]
pub struct WorkBasedProgram {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WorkBasedProgram {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of inserting at the beginning if an ordered collection.
#[derive()]
pub struct PrependAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PrependAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A park.
#[derive()]
pub struct Park {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Park {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A collection of music tracks in playlist form.
#[derive()]
pub struct MusicPlaylist {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MusicPlaylist {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A delivery method is a standardized procedure for transferring the product or service to the destination of fulfillment chosen by the customer. Delivery methods are characterized by the means of transportation used, and by the organization or group that is the contracting party for the sending organization or person.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#DeliveryModeDirectDownload\n* http://purl.org/goodrelations/v1#DeliveryModeFreight\n* http://purl.org/goodrelations/v1#DeliveryModeMail\n* http://purl.org/goodrelations/v1#DeliveryModeOwnFleet\n* http://purl.org/goodrelations/v1#DeliveryModePickUp\n* http://purl.org/goodrelations/v1#DHL\n* http://purl.org/goodrelations/v1#FederalExpress\n* http://purl.org/goodrelations/v1#UPS
///         
#[derive()]
pub struct DeliveryMethod {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DeliveryMethod {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Reserving a concrete object.\n\nRelated actions:\n\n* [[ScheduleAction]]: Unlike ScheduleAction, ReserveAction reserves concrete objects (e.g. a table, a hotel) towards a time slot / spatial allocation.
#[derive()]
pub struct ReserveAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReserveAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An article, such as a news article or piece of investigative report. Newspapers and magazines have articles of many different types and this is intended to cover them all.\n\nSee also [blog post](http://blog.schema.org/2014/09/schemaorg-support-for-bibliographic_2.html).
#[derive()]
pub struct Article {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Article {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Protein is here used in its widest possible definition, as classes of amino acid based molecules. Amyloid-beta Protein in human (UniProt P05067), eukaryota (e.g. an OrthoDB group) or even a single molecule that one can point to are all of type :Protein. A protein can thus be a subclass of another protein, e.g. :Protein as a UniProt record can have multiple isoforms inside it which would also be :Protein. They can be imagined, synthetic, hypothetical or naturally occurring.
#[derive()]
pub struct Protein {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Protein {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A body of water, such as a sea, ocean, or lake.
#[derive()]
pub struct BodyOfWater {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BodyOfWater {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A medical procedure involving an incision with instruments; performed for diagnose, or therapeutic purposes.
#[derive()]
pub struct SurgicalProcedure {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SurgicalProcedure {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A subway station.
#[derive()]
pub struct SubwayStation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SubwayStation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A fast-food restaurant.
#[derive()]
pub struct FastFoodRestaurant {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for FastFoodRestaurant {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A parking lot or other parking facility.
#[derive()]
pub struct ParkingFacility {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ParkingFacility {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Beach.
#[derive()]
pub struct Beach {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Beach {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A bus stop.
#[derive()]
pub struct BusStop {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BusStop {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A short segment/part of a video game.
#[derive()]
pub struct VideoGameClip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VideoGameClip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A type of Bank Account with a main purpose of depositing funds to gain interest or other benefits.
#[derive()]
pub struct DepositAccount {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DepositAccount {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A structured value representing a price or price range. Typically, only the subclasses of this type are used for markup. It is recommended to use [[MonetaryAmount]] to describe independent amounts of money such as a salary, credit card limits, etc.
#[derive()]
pub struct PriceSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PriceSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A photograph.
#[derive()]
pub struct Photograph {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Photograph {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of producing a balanced opinion about the object for an audience. An agent reviews an object with participants resulting in a review.
#[derive()]
pub struct ReviewAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for ReviewAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Bank or credit union.
#[derive()]
pub struct BankOrCreditUnion {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BankOrCreditUnion {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A geographical region, typically under the jurisdiction of a particular government.
#[derive()]
pub struct AdministrativeArea {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AdministrativeArea {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any complaint sensed and expressed by the patient (therefore defined as subjective)  like stomachache, lower-back pain, or fatigue.
#[derive()]
pub struct MedicalSymptom {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalSymptom {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A GeoCircle is a GeoShape representing a circular geographic area. As it is a GeoShape
///           it provides the simple textual property 'circle', but also allows the combination of postalCode alongside geoRadius.
///           The center of the circle can be indicated via the 'geoMidpoint' property, or more approximately using 'address', 'postalCode'.
///        
#[derive()]
pub struct GeoCircle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GeoCircle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A garden store.
#[derive()]
pub struct GardenStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GardenStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A service for a vehicle for hire with a driver for local travel. Fares are usually calculated based on distance traveled.
#[derive()]
pub struct TaxiService {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TaxiService {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An internet cafe.
#[derive()]
pub struct InternetCafe {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for InternetCafe {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A business entity type is a conceptual entity representing the legal form, the size, the main line of business, the position in the value chain, or any combination thereof, of an organization or business person.\n\nCommonly used values:\n\n* http://purl.org/goodrelations/v1#Business\n* http://purl.org/goodrelations/v1#Enduser\n* http://purl.org/goodrelations/v1#PublicInstitution\n* http://purl.org/goodrelations/v1#Reseller
/// 	  
#[derive()]
pub struct BusinessEntityType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BusinessEntityType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A plumbing service.
#[derive()]
pub struct Plumber {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Plumber {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A property, used to indicate attributes and relationships of some Thing; equivalent to rdf:Property.
#[derive()]
pub struct Property {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Property {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of playing a video game.
#[derive()]
pub struct PlayGameAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PlayGameAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of granting permission to an object.
#[derive()]
pub struct AuthorizeAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AuthorizeAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An online or virtual location for attending events. For example, one may attend an online seminar or educational event. While a virtual location may be used as the location of an event, virtual locations should not be confused with physical locations in the real world.
#[derive()]
pub struct VirtualLocation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VirtualLocation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An audio file.
#[derive()]
pub struct AudioObject {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AudioObject {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A short radio program or a segment/part of a radio program.
#[derive()]
pub struct RadioClip {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RadioClip {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A bike store.
#[derive()]
pub struct BikeStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BikeStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A motorcycle dealer.
#[derive()]
pub struct MotorcycleDealer {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MotorcycleDealer {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A zoo.
#[derive()]
pub struct Zoo {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Zoo {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An car dealership.
#[derive()]
pub struct AutoDealer {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AutoDealer {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An indication for treating an underlying condition, symptom, etc.
#[derive()]
pub struct TreatmentIndication {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TreatmentIndication {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A diet restricted to certain foods or preparations for cultural, religious, health or lifestyle reasons. 
#[derive()]
pub struct RestrictedDiet {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for RestrictedDiet {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An [[OpinionNewsArticle]] is a [[NewsArticle]] that primarily expresses opinions rather than journalistic reporting of news and events. For example, a [[NewsArticle]] consisting of a column or [[Blog]]/[[BlogPosting]] entry in the Opinions section of a news publication. 
#[derive()]
pub struct OpinionNewsArticle {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for OpinionNewsArticle {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The publication format of the book.
#[derive()]
pub struct BookFormatType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for BookFormatType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A medical laboratory that offers on-site or off-site diagnostic services.
#[derive()]
pub struct DiagnosticLab {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DiagnosticLab {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An auto parts store.
#[derive()]
pub struct AutoPartsStore {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AutoPartsStore {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A gas station.
#[derive()]
pub struct GasStation {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GasStation {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Represents the collection of all sports organizations, including sports teams, governing bodies, and sports associations.
#[derive()]
pub struct SportsOrganization {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for SportsOrganization {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of posing a question / favor to someone.\n\nRelated actions:\n\n* [[ReplyAction]]: Appears generally as a response to AskAction.
#[derive()]
pub struct AskAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AskAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A recipe. For dietary restrictions covered by the recipe, a few common restrictions are enumerated via [[suitableForDiet]]. The [[keywords]] property can also be used to add more detail.
#[derive()]
pub struct Recipe {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Recipe {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Event type: Theater performance.
#[derive()]
pub struct TheaterEvent {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TheaterEvent {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// NLNonprofitType: Non-profit organization type originating from the Netherlands.
#[derive()]
pub struct NLNonprofitType {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for NLNonprofitType {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A specific and exact (byte-for-byte) version of a [[VideoObject]]. Two byte-for-byte identical files, for the purposes of this type, considered identical. If they have different embedded metadata the files will differ. Different external facts about the files, e.g. creator or dateCreated that aren't represented in their actual content, do not affect this notion of identity.
#[derive()]
pub struct VideoObjectSnapshot {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for VideoObjectSnapshot {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Design models for observational medical studies. Enumerated type.
#[derive()]
pub struct MedicalObservationalStudyDesign {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalObservationalStudyDesign {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The price for the delivery of an offer using a particular delivery method.
#[derive()]
pub struct DeliveryChargeSpecification {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for DeliveryChargeSpecification {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of capturing still images of objects using a camera.
#[derive()]
pub struct PhotographAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PhotographAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A general contractor.
#[derive()]
pub struct GeneralContractor {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for GeneralContractor {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An application programming interface accessible over Web/Internet technologies.
#[derive()]
pub struct WebAPI {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for WebAPI {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// An item used as either a tool or supply when performing the instructions for how to achieve a result.
#[derive()]
pub struct HowToItem {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HowToItem {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Car repair, sales, or parts.
#[derive()]
pub struct AutomotiveBusiness {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for AutomotiveBusiness {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Multicellular parasite that causes an infection.
#[derive()]
pub struct MulticellularParasite {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MulticellularParasite {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Quantities such as distance, time, mass, weight, etc. Particular instances of say Mass are entities like '3 kg' or '4 milligrams'.
#[derive()]
pub struct Quantity {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Quantity {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A tennis complex.
#[derive()]
pub struct TennisComplex {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for TennisComplex {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Any collection of tests commonly ordered together.
#[derive()]
pub struct MedicalTestPanel {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for MedicalTestPanel {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A single season of a podcast. Many podcasts do not break down into separate seasons. In that case, PodcastSeries should be used.
#[derive()]
pub struct PodcastSeason {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for PodcastSeason {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The act of providing an object under an agreement that it will be returned at a later date. Reciprocal of BorrowAction.\n\nRelated actions:\n\n* [[BorrowAction]]: Reciprocal of LendAction.
#[derive()]
pub struct LendAction {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for LendAction {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// The place where a person lives.
#[derive()]
pub struct Residence {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Residence {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A permit issued by an organization, e.g. a parking pass.
#[derive()]
pub struct Permit {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Permit {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A health club.
#[derive()]
pub struct HealthClub {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for HealthClub {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// A News/Media organization such as a newspaper or TV station.
#[derive()]
pub struct NewsMediaOrganization {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for NewsMediaOrganization {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}



/// Quantity: Duration (use [ISO 8601 duration format](http://en.wikipedia.org/wiki/ISO_8601)).
#[derive()]
pub struct Duration {
    pub wip: Vec<zap>,
    pub wop: Vec<zip>,
    pub wup: Vec<zoup>,
    pub sub_class: SubClass,
}

impl Schema for Duration {
    fn new() -> Self {
        Self {
            props_init,
            sub_class_init
        }
    }           

    fn add_property(&mut self, name: String, value: String) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => {},
            _ => return Err(Error::InvalidProperty),
        }
        Ok(())
    }

    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error> {
        match name.to_lowercase().as_str() {
            "" => match item {
                Types::Date(date) => {
                    Ok(())
                }
                _ => Err(Error::InvalidType),
            },
            _ => {
                {sub_class_test}
            },
        }
    }
  
}
