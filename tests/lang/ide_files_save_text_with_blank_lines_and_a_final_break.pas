program ide_files_save_text_with_blank_lines_and_a_final_break;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('a~~ b ~~');
  EditLoop;
  Report('BL.PAS');
end.
