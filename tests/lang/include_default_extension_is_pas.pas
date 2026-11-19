program IncludeDefaultExtensionIsPas;
{ a name without extension means NAME.PAS }
{$I PIECE}
begin
  WriteLn(Piece);
end.
