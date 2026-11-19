program ide_errors_texts_of_known_codes;
uses Crt, Errors;

procedure Show(Code: Integer);
begin
  WriteLn(Code, ' [', ErrorText(Code), ']');
end;

begin
  Show(2);
  Show(3);
  Show(5);
  Show(33);
  Show(64);
  Show(85);
  Show(93);
  Show(133);
  Show(200);
end.
