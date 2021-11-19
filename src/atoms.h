#ifndef _ATOMS_H
#define _ATOMS_H

#include <X11/Xutil.h>

/* EWMH atoms */
enum {
	NetSupported, NetWMName, NetWMState, NetWMCheck,
	NetWMFullscreen, NetActiveWindow, NetWMWindowType, NetWMWindowTypeDialog,
	NetClientList, NetLast,
};

/* default atoms */
enum { WMProtocols, WMDelete, WMState, WMTakeFocus, WMLast };

typedef struct Atoms {
	Atom wmatom[WMLast], netatom[NetLast], utf8string;
} *Atoms;

Atoms atoms_create(Display *dpy);
void atoms_destroy(Atoms atoms);

#endif // _ATOMS_H
