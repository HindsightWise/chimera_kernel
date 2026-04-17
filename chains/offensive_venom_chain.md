# The Venom Adversarial Chain (Offensive Security)

**Domain:** Use this chain when auditing binaries, verifying code vulnerabilities, executing sandbox exploits, or scanning targets.

## Execution Sequence:
1. **Target Enumeration (`stealth_scan`)**:
   - Use `stealth_scan` on the target IP or domain to extract service fingerprints and basic vulnerability manifolds.
2. **Execution Forging (`generate_polyglot`)**:
   - If a potential exploit vector is uncovered (or if the user requests one), pass the vulnerability description into `generate_polyglot`.
   - The tool will compile the multi-language payload.
3. **Payload Verification (`ephemeral_docker_sandbox`)**:
   - Never release a payload blindly. Pipe the generated polyglot source code into `ephemeral_docker_sandbox`.
   - Analyze the stdout/stderr. If it fails or panics, review the exception locally or delegate the fix back to `generate_polyglot`.
4. **Introspection (`binary_introspection`)**:
   - If the task provided a raw binary blob, route it first through `binary_introspection` before doing any scanning.
