/// Identifies every managed panel in the application.
/// Add a new variant here when you add a new panel — that's the only place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanelId {
    Console,
    Field,
    FieldPlayback,
    Graph,
    RawPlayback,
    RawSerial,
    SerialSettings,
    WindowLayouts,
}
