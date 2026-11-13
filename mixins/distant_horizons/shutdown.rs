// Close the thread-local DH engine before process/thread-local destruction.
// LGPL-3.0-only; see LICENSE for the full terms.
fn distant_horizons_shutdown(game: &mut crate::game::Game<'_>) {
    crate::game::distant_horizons_bridge::shutdown(game);
}
