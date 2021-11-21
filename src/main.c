#include "main.h"

#include "dwm.h"

#include <locale.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void logger(const char *level, const char *fmt, ...);
static void logger_perror(const char *level, const char *fmt, ...);

int main(int argc, char *argv[])
{
	if (argc == 2 && strcmp(argv[1], "-v") == 0) {
		fputs("polytreewm-"VERSION"\n", stderr);
		exit(EXIT_SUCCESS);
	}

	if (argc != 1) {
		fprintf(stderr, "usage: polytreewm [-v]\n");
		exit(EXIT_FAILURE);
	}

	if (!setlocale(LC_CTYPE, "") || !dwm_has_locale_support()) {
		warning("no locale support");
	}

	return dwm_main(argc, argv);
}

void logger(const char *const level, const char *const fmt, ...)
{
	fprintf(stderr, "%s: ", level);

	va_list ap;
	va_start(ap, fmt);
	vfprintf(stderr, fmt, ap);
	va_end(ap);

	fputc('\n', stderr);
}

void logger_perror(const char *const level, const char *const fmt, ...)
{
	fprintf(stderr, "%s: ", level);

	va_list ap;
	va_start(ap, fmt);
	vfprintf(stderr, fmt, ap);
	va_end(ap);

	fprintf(stderr, ": ");
	perror(NULL);
}

void fatal(const char *const fmt, ...)
{
	va_list ap;
	va_start(ap, fmt);
	logger("ERROR", fmt, ap);
	va_end(ap);

	exit(EXIT_FAILURE);
}

void fatal_perror(const char* const fmt, ...)
{
	va_list ap;
	va_start(ap, fmt);
	logger_perror("ERROR", fmt, ap);
	va_end(ap);

	exit(EXIT_FAILURE);
}

void fatal_nodie(const char *const fmt, ...)
{
	va_list ap;
	va_start(ap, fmt);
	logger("ERROR", fmt, ap);
	va_end(ap);
}

void fatal_perror_nodie(const char *const fmt, ...)
{
	va_list ap;
	va_start(ap, fmt);
	logger_perror("ERROR", fmt, ap);
	va_end(ap);
}

void warning(const char *const fmt, ...)
{
	va_list ap;
	va_start(ap, fmt);
	logger("WARN", fmt, ap);
	va_end(ap);
}

void warning_perror(const char *const fmt, ...)
{
	va_list ap;
	va_start(ap, fmt);
	logger_perror("WARN", fmt, ap);
	va_end(ap);
}
