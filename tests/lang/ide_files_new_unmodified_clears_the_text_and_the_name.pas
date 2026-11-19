program ide_files_new_unmodified_clears_the_text_and_the_name;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('abc');
  EditLoop;
  Report('GOOD.TXT');
end.
