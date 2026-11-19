program ide_files_open_keeps_blank_lines_and_trailing_blanks_and_the_missing_final_break;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('BLANKS.TXT');
end.
