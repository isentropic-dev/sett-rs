pub(super) struct Engine {
    fluid: Fluid,
    components: Components,
}
struct Fluid {}
struct Components {
    chx: ColdHeatExchanger,
    hhx: HotHeatExchanger,
    regen: Regenerator,
    ws: WorkingSpaces,
}
struct ColdHeatExchanger {}
struct HotHeatExchanger {}
struct Regenerator {}
struct WorkingSpaces {}
