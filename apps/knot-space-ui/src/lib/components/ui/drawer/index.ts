import { Drawer as DrawerPrimitive } from "vaul-svelte";

const Root = DrawerPrimitive.Root;
const Trigger = DrawerPrimitive.Trigger;
const Portal = DrawerPrimitive.Portal;
const Close = DrawerPrimitive.Close;

export {
	Root,
	Trigger,
	Portal,
	Close,
	//
	Root as Drawer,
	Trigger as DrawerTrigger,
	Portal as DrawerPortal,
	Close as DrawerClose,
};

export { default as DrawerContent } from "./drawer-content.svelte";
export { default as DrawerDescription } from "./drawer-description.svelte";
export { default as DrawerFooter } from "./drawer-footer.svelte";
export { default as DrawerHeader } from "./drawer-header.svelte";
export { default as DrawerOverlay } from "./drawer-overlay.svelte";
export { default as DrawerTitle } from "./drawer-title.svelte";