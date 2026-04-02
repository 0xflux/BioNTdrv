# POC CVE-2025-0288

This is a fully weaponised Rust Proof of Concept exploiting a physical memory read & virtual memory write primitive in the known
vulnerable driver [BioNTdrv.sys](https://www.loldrivers.io/drivers/e6378671-986d-42a1-8e7a-717117c83751/) which elevates the token of a process **from** Local Admin to SYSTEM through DKOM. 

[CVE-2025-0288](https://nvd.nist.gov/vuln/detail/CVE-2025-0288) is listed on MITRE as:

```
Various Paragon Software products contain an arbitrary kernel memory vulnerability within biontdrv.sys, facilitated by the 
memmove function, which does not validate or sanitize user controlled input, allowing an attacker the ability to write arbitrary 
kernel memory and perform privilege escalation.
```

The main exploit was fully developed by Claude Code, see my [X Post]([url](https://x.com/0xfluxsec/status/2039756031396303349)) describing my analysis.

THIS DRIVER IS ON THE KNOWN BLOCKLIST SO THIS IS POSTED FOR EDUCATIONAL PURPOSES ONLY TO AID CONVERSATIONS AROUND
THE USE OF AI FOR EXPLOIT DEVELOPMENT.

To see the DKOM exploit by Claude (not the small POC I wrote which is in `main.rs`), see the source file: [claude_exploit.rs](https://github.com/0xflux/BioNTdrv/blob/master/src/claude_exploit.rs) in this project. **Note** this requires Admin, so it is a **high integrity** -> **system** elevation only.

### Proof:
 
 ![Proof of POC](proof.jpg)

## Constraints

This LPE is `Local Admin` -> `SYSTEM`, therefore must be run from a high integrity session.

## Misc

This is a significant improvement over what GPT-5.4 produced which was only the discovery of the vulnerable driver. It refused to write an exploit. To see the small POC exploit I wrote which simply abuses the read/write primitive, check `main.rs`.