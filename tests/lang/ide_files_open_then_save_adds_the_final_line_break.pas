program ide_files_open_then_save_adds_the_final_line_break;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('BLANKS.TXT');
end.
