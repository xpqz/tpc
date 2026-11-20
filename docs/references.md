# References

- **Turbo Pascal Reference Guide, Version 5.0** (Borland, 1989), OCR text on archive.org:
  https://archive.org/download/bitsavers_borlandturVersion5.0ReferenceGuide1989_16033215/Turbo_Pascal_Version_5.0_Reference_Guide_1989_djvu.txt
  Primary source for TP5 semantics. Ch. 3 "Types" (Integer Types), Ch. 6 "Expressions".
- **Turbo Pascal Language Guide, Version 7.0** (Borland, 1992), archive.org item
  `bitsavers_borlandturVersion7.0LanguageGuide1992_10480217`. Secondary; TP5 wins on conflict.
- **Turbo Pascal 5.5 Norton Guide** (third-party conversion), error tables:
  https://discmaster.textfiles.com/file/9297/Cream%20of%20the%20Crop%201.iso/CAD/G2P55.ARJ/G2P55.NG/ng2f7d0.html
  Basis of `docs/tp-errors.md`.
- **Turbo Pascal 5.0 itself** (Borland, 1988), the oracle: WinWorld,
  https://winworldpc.com/product/turbo-pascal/5x, "Borland Turbo Pascal 5.0 (5.25-360k)".
  Extract `TPC.EXE` and `TURBO.TPL` from disk 1 into `tp5/` at the top of the checkout
  (git-ignored), or set `TP5_HOME`; `tools/tp5.sh` runs them under DOSBox-X. Not committed to the repo.
- **Free Pascal 3.2.2** `-Mtp`, installed locally: oracle for program output except where
  `docs/decisions.md` or `docs/deviations.md` says otherwise.
