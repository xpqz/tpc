unit Probe;
interface
function UnitSees: string;
implementation
function UnitSees: string;
begin
{$IFDEF FROM_PROGRAM}
  UnitSees := 'unit sees it';
{$ELSE}
  UnitSees := 'unit does not see it';
{$ENDIF}
end;
end.
