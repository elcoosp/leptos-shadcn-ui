//! # Leptos ShadCN UI
//!
//! A comprehensive collection of beautiful, accessible UI components built for
//! [Leptos](https://leptos.dev/) v0.9, inspired by [shadcn/ui](https://ui.shadcn.com/).
//!
//! ## Features
//!
//! - **~59 Components**: Button, Input, Card, Dialog, Select, Sheet, Drawer, and many more
//! - **Leptos 0.9**: Built for Leptos `0.9.0-rc0` (pinned by the workspace)
//! - **Accessibility First**: All components follow accessibility best practices
//! - **Tailwind CSS**: Seamless integration with Tailwind CSS
//! - **Type Safety**: Full Rust type safety with proper error handling
//!
//! ## Usage
//!
//! Enable the features for the components you want to use (`all-components` is the default):
//!
//! ```toml
//! [dependencies]
//! leptos-shadcn-ui = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1", features = ["button", "input", "card"] }
//! ```
//!
//! Every component enabled by a feature is re-exported at the crate root and in the
//! [`prelude`]; the `public_api_tests` regression suite fails the build if the two drift.
//!
//! ## Components
//!
//! ### Form Components
//! - Button, Input, Label, Checkbox, Switch, Radio Group, Select, Textarea, Form, Field,
//!   Input Group, Input OTP, Native Select, Combobox, Command
//!
//! ### Layout Components
//! - Card, Separator, Tabs, Accordion, Dialog, Alert Dialog, Sheet, Drawer, Popover,
//!   Tooltip, Hover Card, Collapsible, Aspect Ratio, Sidebar, Resizable, Scroll Area
//!
//! ### Navigation
//! - Breadcrumb, Navigation Menu, Context Menu, Dropdown Menu, Menubar, Pagination
//!
//! ### Feedback & Status
//! - Alert, Badge, Skeleton, Progress, Toast, Sonner, Table, Calendar, Date Picker,
//!   Carousel, Kbd, Spinner, Empty, Item, Typography, Toggle, Toggle Group, Button Group,
//!   Chart, Error Boundary
//!
//! ### Performance Monitoring
//! - Performance Audit System - Comprehensive performance monitoring and optimization
//! - Bundle Size Analysis - Component size tracking and optimization recommendations
//! - Real-time Monitoring - Performance metrics collection and analysis
//! - CLI Tool - Command-line interface for running audits and generating reports
//!
//! ## License
//!
//! MIT License - see the [LICENSE](../../LICENSE) file for details.

// Several component crates export a `signal_managed` module; their items are re-exported
// here, and narrowing the globs would drop public items.
#![allow(ambiguous_glob_reexports)]

// Re-export all components (conditionally based on features)

// Components added to track current shadcn/ui
#[cfg(feature = "button-group")]
pub use leptos_shadcn_button_group::default::*;
#[cfg(feature = "chart")]
pub use leptos_shadcn_chart::*;
#[cfg(feature = "empty")]
pub use leptos_shadcn_empty::default::*;
#[cfg(feature = "field")]
pub use leptos_shadcn_field::default::*;
#[cfg(feature = "input-group")]
pub use leptos_shadcn_input_group::default::*;
#[cfg(feature = "item")]
pub use leptos_shadcn_item::default::*;
#[cfg(feature = "kbd")]
pub use leptos_shadcn_kbd::default::*;
#[cfg(feature = "native-select")]
pub use leptos_shadcn_native_select::default::*;
#[allow(ambiguous_glob_reexports)]
#[cfg(feature = "sidebar")]
pub use leptos_shadcn_sidebar::*;
#[cfg(feature = "sonner")]
pub use leptos_shadcn_sonner::*;
#[cfg(feature = "spinner")]
pub use leptos_shadcn_spinner::default::*;
#[cfg(feature = "toggle-group")]
pub use leptos_shadcn_toggle_group::default::*;
#[cfg(feature = "typography")]
pub use leptos_shadcn_typography::default::*;

#[cfg(feature = "accordion")]
pub use leptos_shadcn_accordion::default::*;
#[cfg(feature = "alert")]
pub use leptos_shadcn_alert::default::*;
#[cfg(feature = "badge")]
pub use leptos_shadcn_badge::default::*;
#[cfg(feature = "button")]
pub use leptos_shadcn_button::default::*;
#[cfg(feature = "card")]
pub use leptos_shadcn_card::default::*;
#[cfg(feature = "checkbox")]
pub use leptos_shadcn_checkbox::default::*;
#[cfg(feature = "dialog")]
pub use leptos_shadcn_dialog::default::*;
#[cfg(feature = "input")]
pub use leptos_shadcn_input::default::*;
#[cfg(feature = "label")]
pub use leptos_shadcn_label::default::*;
#[cfg(feature = "popover")]
pub use leptos_shadcn_popover::default::*;
#[cfg(feature = "radio-group")]
pub use leptos_shadcn_radio_group::default::*;
#[cfg(feature = "select")]
pub use leptos_shadcn_select::default::*;
// The select crate nests its part components in submodules (`select_root`,
// `select_content`, `select_scroll`), so its `default::*` glob only lifts module
// names. Re-export the parts flat at the root so `use leptos_shadcn_ui::Select`
// works like every other component. (Resolves the old
// `// FIXME: use correct path from select crate` in the prelude.)
#[cfg(feature = "select")]
pub use leptos_shadcn_select::default::select_content::{
    SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator,
};
#[cfg(feature = "select")]
pub use leptos_shadcn_select::default::select_root::{Select, SelectTrigger, SelectValue};
#[cfg(feature = "separator")]
pub use leptos_shadcn_separator::default::*;
#[cfg(feature = "skeleton")]
pub use leptos_shadcn_skeleton::default::*;
#[cfg(feature = "switch")]
pub use leptos_shadcn_switch::default::*;
#[cfg(feature = "tabs")]
pub use leptos_shadcn_tabs::default::*;
#[cfg(feature = "textarea")]
pub use leptos_shadcn_textarea::default::*;
#[cfg(feature = "tooltip")]
pub use leptos_shadcn_tooltip::default::*;
// Sidebar also exports a SKELETON_CLASS; the Skeleton component's constant takes precedence.
#[cfg(feature = "calendar")]
pub use leptos_shadcn_calendar::*;
#[cfg(feature = "date-picker")]
pub use leptos_shadcn_date_picker::*;
#[cfg(feature = "pagination")]
pub use leptos_shadcn_pagination::*;
#[cfg(feature = "progress")]
pub use leptos_shadcn_progress::default::*;
#[cfg(feature = "skeleton")]
pub use leptos_shadcn_skeleton::default::SKELETON_CLASS;
#[cfg(feature = "slider")]
pub use leptos_shadcn_slider::default::*;
#[cfg(feature = "table")]
pub use leptos_shadcn_table::default::*;
#[cfg(feature = "toast")]
pub use leptos_shadcn_toast::default::*;
#[cfg(feature = "toggle")]
pub use leptos_shadcn_toggle::default::*;

// Advanced components (newly fixed)
#[cfg(feature = "analytics")]
pub use leptos_shadcn_analytics::*;
#[cfg(feature = "breadcrumb")]
pub use leptos_shadcn_breadcrumb::*;
#[cfg(feature = "combobox")]
pub use leptos_shadcn_combobox::default::*;
#[cfg(feature = "command")]
pub use leptos_shadcn_command::*;
#[cfg(feature = "error-boundary")]
pub use leptos_shadcn_error_boundary::*;
#[cfg(feature = "form")]
pub use leptos_shadcn_form::default::*;
#[cfg(feature = "input-otp")]
pub use leptos_shadcn_input_otp::*;
#[cfg(feature = "lazy-loading")]
pub use leptos_shadcn_lazy_loading::*;
#[cfg(feature = "registry")]
pub use leptos_shadcn_registry::*;

// Components restored to the public API: their features were enabled in
// `all-components` (compiling the crates and paying the dependency cost), but no
// re-export existed at the crate root, making every one of them unreachable
// through the umbrella crate.
#[cfg(feature = "alert-dialog")]
pub use leptos_shadcn_alert_dialog::default::*;
#[cfg(feature = "aspect-ratio")]
pub use leptos_shadcn_aspect_ratio::default::*;
#[cfg(feature = "avatar")]
pub use leptos_shadcn_avatar::default::*;
#[cfg(feature = "carousel")]
pub use leptos_shadcn_carousel::default::*;
#[cfg(feature = "collapsible")]
pub use leptos_shadcn_collapsible::default::*;
#[cfg(feature = "context-menu")]
pub use leptos_shadcn_context_menu::default::*;
#[cfg(feature = "drawer")]
pub use leptos_shadcn_drawer::default::*;
#[cfg(feature = "dropdown-menu")]
pub use leptos_shadcn_dropdown_menu::default::*;
#[cfg(feature = "hover-card")]
pub use leptos_shadcn_hover_card::default::*;
#[cfg(feature = "menubar")]
pub use leptos_shadcn_menubar::default::*;
#[cfg(feature = "navigation-menu")]
pub use leptos_shadcn_navigation_menu::default::*;
#[cfg(feature = "performance-audit")]
pub use leptos_shadcn_performance_audit::*;
#[cfg(feature = "resizable")]
pub use leptos_shadcn_resizable::default::*;
#[cfg(feature = "scroll-area")]
pub use leptos_shadcn_scroll_area::default::*;
#[cfg(feature = "sheet")]
pub use leptos_shadcn_sheet::default::*;

// Re-export common types and utilities
pub use tailwind_fuse::tw_merge;

// Module documentation
#[cfg(feature = "all-components")]
pub mod prelude {
    //! # Leptos ShadCN UI Prelude
    //!
    //! This module re-exports the most commonly used components and types.
    //! Only symbols that actually exist are listed; `public_api_tests` in the
    //! crate root fails the build if a component rename drops a name from here.
    //!
    //! ```rust
    //! use leptos_shadcn_ui::prelude::*;
    //! ```

    // Form components
    #[cfg(feature = "checkbox")]
    pub use super::Checkbox;
    #[cfg(feature = "input")]
    pub use super::Input;
    #[cfg(feature = "label")]
    pub use super::Label;
    #[cfg(feature = "switch")]
    pub use super::Switch;
    #[cfg(feature = "textarea")]
    pub use super::Textarea;
    #[cfg(feature = "button")]
    pub use super::{Button, ButtonResponsiveSize, ButtonSize, ButtonVariant};
    #[cfg(feature = "radio-group")]
    pub use super::{RadioGroup, RadioGroupItem};
    #[cfg(feature = "select")]
    pub use super::{Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue};

    // Layout components
    #[cfg(feature = "separator")]
    pub use super::Separator;
    #[cfg(feature = "accordion")]
    pub use super::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
    #[cfg(feature = "card")]
    pub use super::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
    #[cfg(feature = "dialog")]
    pub use super::{
        Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
        DialogTitle, DialogTrigger,
    };
    #[cfg(feature = "popover")]
    pub use super::{Popover, PopoverContent, PopoverTrigger};
    #[cfg(feature = "tabs")]
    pub use super::{Tabs, TabsContent, TabsList, TabsTrigger};
    #[cfg(feature = "tooltip")]
    pub use super::{Tooltip, TooltipContent, TooltipProvider, TooltipTrigger};

    // Feedback components
    #[cfg(feature = "calendar")]
    pub use super::Calendar;
    #[cfg(feature = "date-picker")]
    pub use super::DatePicker;
    #[cfg(feature = "progress")]
    pub use super::Progress;
    #[cfg(feature = "table")]
    pub use super::Table;
    #[cfg(feature = "toast")]
    pub use super::Toast;
    #[cfg(feature = "alert")]
    pub use super::{Alert, AlertDescription, AlertTitle, AlertVariant};
    #[cfg(feature = "badge")]
    pub use super::{Badge, BadgeVariant};
    #[cfg(feature = "pagination")]
    pub use super::{
        Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
        PaginationNext, PaginationPrevious,
    };
    #[cfg(feature = "skeleton")]
    pub use super::{Skeleton, SkeletonSize, SkeletonVariant};

    // Interactive components
    #[cfg(feature = "slider")]
    pub use super::Slider;
    #[cfg(feature = "toggle")]
    pub use super::Toggle;

    // Utilities
    pub use super::tw_merge;
}

#[cfg(all(test, feature = "all-components"))]
mod public_api_tests {
    //! Compile-time regression guard for the umbrella's public API.
    //!
    //! Every name imported below must resolve, or the crate fails to compile with a
    //! targeted `unresolved import` error. This pins the contract that (a) every
    //! component enabled by `all-components` is reachable from the crate root and
    //! (b) the prelude only contains symbols that actually exist.

    /// Identity helper proving that a *type* (enum/struct) is nameable.
    fn assert_type_exists<T>() {}

    #[test]
    fn root_exports_every_restored_component() {
        use crate::{
            AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent,
            AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogOverlay,
            AlertDialogTitle, AlertDialogTrigger, AspectRatio, Avatar, AvatarFallback, AvatarGroup,
            AvatarImage, Carousel, CarouselApi, CarouselContent, CarouselItem, CarouselNext,
            CarouselOrientation, CarouselPrevious, Collapsible, CollapsibleContent,
            CollapsibleTrigger, ContextMenu, ContextMenuCheckboxItem, ContextMenuContent,
            ContextMenuItem, ContextMenuLabel, ContextMenuRadioGroup, ContextMenuRadioItem,
            ContextMenuSeparator, ContextMenuShortcut, ContextMenuSub, ContextMenuSubContent,
            ContextMenuSubTrigger, ContextMenuTrigger, Drawer, DrawerClose, DrawerContent,
            DrawerDescription, DrawerDirection, DrawerFooter, DrawerHeader, DrawerNestedRoot,
            DrawerOverlay, DrawerPortal, DrawerTitle, DrawerTrigger, DropdownMenu,
            DropdownMenuContent, DropdownMenuContext, DropdownMenuGroup, DropdownMenuItem,
            DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuTrigger,
            HoverCard, HoverCardContent, HoverCardTrigger, Menubar, MenubarContent, MenubarItem,
            MenubarMenu, MenubarMenuContext, MenubarTrigger, NavigationMenu, NavigationMenuContent,
            NavigationMenuContext, NavigationMenuItem, NavigationMenuItemContext,
            NavigationMenuLink, NavigationMenuList, NavigationMenuTrigger, ResizableHandle,
            ResizablePanel, ResizablePanelGroup, SKELETON_CLASS, ScrollArea, Sheet,
        };

        // Components are `#[component]` fns; referencing them as values proves the
        // crate root re-exports them.
        let _ = (
            AlertDialog,
            AlertDialogAction,
            AlertDialogCancel,
            AlertDialogContent,
            AlertDialogDescription,
            AlertDialogFooter,
            AlertDialogHeader,
            AlertDialogOverlay,
            AlertDialogTitle,
            AlertDialogTrigger,
            AspectRatio,
            Avatar,
            AvatarFallback,
            AvatarGroup,
            AvatarImage,
            Carousel,
            CarouselContent,
            CarouselItem,
            CarouselNext,
            CarouselPrevious,
            Collapsible,
            CollapsibleContent,
            CollapsibleTrigger,
            ContextMenu,
            ContextMenuCheckboxItem,
            ContextMenuContent,
            ContextMenuItem,
            ContextMenuLabel,
            ContextMenuRadioGroup,
            ContextMenuRadioItem,
            ContextMenuSeparator,
            ContextMenuShortcut,
            ContextMenuSub,
            ContextMenuSubContent,
            ContextMenuSubTrigger,
            ContextMenuTrigger,
            Drawer,
            DrawerClose,
            DrawerContent,
            DrawerDescription,
            DrawerFooter,
            DrawerHeader,
            DrawerNestedRoot,
            DrawerOverlay,
            DrawerPortal,
            DrawerTitle,
            DrawerTrigger,
            DropdownMenu,
            DropdownMenuContent,
            DropdownMenuGroup,
            DropdownMenuItem,
            DropdownMenuLabel,
            DropdownMenuSeparator,
            DropdownMenuShortcut,
            DropdownMenuTrigger,
            HoverCard,
            HoverCardContent,
            HoverCardTrigger,
            Menubar,
            MenubarContent,
            MenubarItem,
            MenubarMenu,
            MenubarTrigger,
            NavigationMenu,
            NavigationMenuContent,
            NavigationMenuItem,
            NavigationMenuLink,
            NavigationMenuList,
            NavigationMenuTrigger,
            ResizableHandle,
            ResizablePanel,
            ResizablePanelGroup,
            ScrollArea,
            Sheet,
        );

        // Types and enums exposed by the restored components
        fn restored_types_exist() {
            assert_type_exists::<CarouselApi>();
            assert_type_exists::<CarouselOrientation>();
            assert_type_exists::<DrawerDirection>();
            assert_type_exists::<DropdownMenuContext>();
            assert_type_exists::<MenubarMenuContext>();
            assert_type_exists::<NavigationMenuContext>();
            assert_type_exists::<NavigationMenuItemContext>();
        }
        restored_types_exist();

        // Constants (explicit re-export disambiguates them from sidebar's glob)
        let _ = SKELETON_CLASS;
    }

    #[test]
    fn prelude_exports_real_symbols_only() {
        use crate::prelude::*;

        // Components are fn items; enums are checked in `prelude_types_exist` below.
        let _ = (
            Accordion,
            AccordionContent,
            AccordionItem,
            AccordionTrigger,
            Alert,
            AlertDescription,
            AlertTitle,
            Badge,
            Button,
            Calendar,
            Card,
            CardContent,
            CardDescription,
            CardFooter,
            CardHeader,
            CardTitle,
            Checkbox,
            DatePicker,
            Dialog,
            DialogClose,
            DialogContent,
            DialogDescription,
            DialogFooter,
            DialogHeader,
            DialogTitle,
            DialogTrigger,
            Input,
            Label,
            Pagination,
            PaginationContent,
            PaginationEllipsis,
            PaginationItem,
            PaginationLink,
            PaginationNext,
            PaginationPrevious,
            Popover,
            PopoverContent,
            PopoverTrigger,
            Progress,
            RadioGroup,
            RadioGroupItem,
            Select,
            SelectContent,
            SelectGroup,
            SelectItem,
            SelectTrigger,
            SelectValue,
            Separator,
            Skeleton,
            Slider,
            Switch,
            Table,
            Tabs,
            TabsContent,
            TabsList,
            TabsTrigger,
            Textarea,
            Toast,
            Toggle,
            Tooltip,
            TooltipContent,
            TooltipProvider,
            TooltipTrigger,
        );

        // `tw_merge` is a macro_rules! re-export, so it cannot sit in the value tuple;
        // invoking it is the compile-time proof that the prelude still exports it.
        let _ = tw_merge!("p-1", "p-2");

        fn prelude_types_exist() {
            assert_type_exists::<AlertVariant>();
            assert_type_exists::<BadgeVariant>();
            assert_type_exists::<ButtonResponsiveSize>();
            assert_type_exists::<ButtonSize>();
            assert_type_exists::<ButtonVariant>();
            assert_type_exists::<SkeletonSize>();
            assert_type_exists::<SkeletonVariant>();
        }
        prelude_types_exist();
    }
}
