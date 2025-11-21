package exitcode

type Exitcode int

const (
	Success Exitcode = iota
	Interrupt
	VersionError
)
