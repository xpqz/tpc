program ide_files_open_one_character_too_large_empties_the_text_and_drops_the_name;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('old');
  EditLoop;
  Report('GOOD.TXT');
end.
