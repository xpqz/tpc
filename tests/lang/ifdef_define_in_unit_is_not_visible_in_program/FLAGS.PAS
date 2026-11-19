unit Flags;
interface
{$DEFINE FROM_UNIT}
function InUnit: string;
implementation
function InUnit: string;
begin
{$IFDEF FROM_UNIT}
  InUnit := 'defined in unit';
{$ELSE}
  InUnit := 'not defined in unit';
{$ENDIF}
end;
end.
