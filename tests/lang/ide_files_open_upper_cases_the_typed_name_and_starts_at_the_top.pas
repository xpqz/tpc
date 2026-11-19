program ide_files_open_upper_cases_the_typed_name_and_starts_at_the_top;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('a~b');
  EditLoop;
  Report('GOOD.TXT');
end.
