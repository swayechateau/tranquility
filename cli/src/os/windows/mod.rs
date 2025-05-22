
// ────────────── Windows ──────────────
pub fn install() {
    // update winget via package manager
    let pm = PackageManager::new();
    pm.update();
}
