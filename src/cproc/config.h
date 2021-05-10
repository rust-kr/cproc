static const char target[]               = "";
static const char *const startfiles[]    = {""};
static const char *const endfiles[]      = {""};
static const char *const preprocesscmd[] = {
	"$DEFAULT_PREPROCESSOR", "-P",
	/* clear preprocessor GNU C version */
	"-U", "__GNUC__",
	"-U", "__GNUC_MINOR__",
	/* we don't yet support these optional features */
	"-D", "__STDC_NO_ATOMICS__",
	"-D", "__STDC_NO_COMPLEX__",
	"-D", "__STDC_NO_VLA__",
	"-U", "__SIZEOF_INT128__",
	/* ignore attributes and extension markers */
	"-D", "__attribute__(x)=",
	"-D", "__extension__="};
static const char *const codegencmd[]    = {""};
static const char *const assemblecmd[]   = {""};
static const char *const linkcmd[]       = {""};