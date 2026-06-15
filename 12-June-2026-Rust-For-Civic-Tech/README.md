# Rust For Civic Tech

Presented by Collins Muriuki ([`@c12i`](https://c12i.xyz))

**Date**: 12th June 2026

## About the Talk

A tour through three open-source Rust projects that grew into one civic data
platform for Kenya's parliamentary data:

- **Bunge Bits**: transcribes parliamentary livestreams from YouTube using
  Whisper and LLM summarization
- **odnelazm**: a unified scraper, CLI, and MCP server for Kenya's official
  Hansard records, covering both the current site and the archive back to 2006
- **Bunge Hub**: the web frontend that puts all of this in front of citizens

The talk covers scraping messy real-world HTML, structuring it into a
Postgres-backed ingest pipeline, exposing it over a CLI and an MCP server, and
enriching it with local LLMs, all in production Rust.

## Presentation Slides

Live: https://rust-for-civic-tech-talk.vercel.app/1
PDF: [rust-for-civic-tech.pdf](./rust-for-civic-tech.pdf)

_Powered by [sli.dev](https://sli.dev)_

## Links

- Mwananchi Tech: https://mwananchi.tech
- Bunge Hub: https://bunge-hub.mwananchi.tech
- Bunge Bits: https://bunge-bits.mwananchi.tech
- GitHub: https://github.com/mwananchi-tech
  - [odnelazm](https://github.com/mwananchi-tech/odnelazm)
  - [bunge-bits](https://github.com/mwananchi-tech/bunge-bits)

## LICENSE

All work in this repository is released under [Apache-2.0](../LICENSE)
