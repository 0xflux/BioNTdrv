# POC CVE-2025-0288

This is a fully weaponised Rust Proof of Concept exploiting a physical memory read & virtual memory write primitive in the known
vulnerable driver [BioNTdrv.sys](https://www.loldrivers.io/drivers/e6378671-986d-42a1-8e7a-717117c83751/) to elevate privilege to SYSTEM.

The main exploit was fully developed by Claude Code, see my [X Post]([url](https://x.com/0xfluxsec/status/2039756031396303349)) describing my analysis.

THIS DRIVER IS ON THE KNOWN BLOCKLIST SO THIS IS POSTED FOR EDUCATIONAL PURPOSES ONLY TO AID CONVERSATIONS AROUND
THE USE OF AI FOR EXPLOIT DEVELOPMENT.

To see the FULL local privilege escallation exploit, see the source file: [claude_exploit.rs]([url](https://github.com/0xflux/BioNTdrv/blob/master/src/claude_exploit.rs)) in this project.

### Proof:
 
 ![Proof of POC](proof.jpg)