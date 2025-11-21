package cli

import (
	"context"

	"github.com/anttiharju/rust-starter/internal/buildinfo"
	"github.com/anttiharju/rust-starter/internal/exitcode"
)

func Start(_ context.Context, info buildinfo.BuildInfo, _ []string) exitcode.Exitcode {
	return buildinfo.Print(info)
}
