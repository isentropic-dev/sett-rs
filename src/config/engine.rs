pub(super) struct Engine {
    fluid: Fluid,
    components: Components,
}
struct Fluid {}
struct Components {
    chx: ColdHeatExchanger,
    hhx: HotHeatExchanger,
    regenerator: Regenerator,
    working_spaces: WorkingSpaces,
}
struct ColdHeatExchanger {}
struct HotHeatExchanger {}
struct Regenerator {}
struct WorkingSpaces {}
