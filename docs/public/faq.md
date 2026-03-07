---
status: stable
---

# Frequently Asked Questions

## General

**Q: What is AgenticEvolve?**
A: A pattern library engine that crystallizes verified code patterns for reuse by AI agents.

**Q: Do I need other Agentra sisters?**
A: No. AgenticEvolve is fully standalone. Sister integration is optional.

**Q: What languages are supported for crystallization?**
A: Rust and Python are first-class. Other languages work with generic template extraction.

## Patterns

**Q: How are patterns stored?**
A: In `.aevolve` binary files with versioned format, indexed for fast lookup.

**Q: Can I share patterns between projects?**
A: Yes. Export/import commands support JSON interchange.

**Q: How does confidence scoring work?**
A: Confidence combines template quality (variable detection, structure) with usage history (success rate, frequency).

## MCP

**Q: How do I connect AgenticEvolve to my IDE?**
A: The installer auto-configures Claude Desktop, Cursor, Windsurf, VS Code, and other MCP clients.

**Q: Can I use AgenticEvolve without MCP?**
A: Yes. The CLI works standalone without any MCP client.

## Performance

**Q: How fast is pattern matching?**
A: Signature matching runs in ~45 microseconds for 100 patterns.

**Q: How large can the pattern store get?**
A: Tested with 10,000+ patterns. The index scales well.
