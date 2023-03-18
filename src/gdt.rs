use lazy_static::lazy_static;
use x86_64::{
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr, registers::segmentation::{CS, Segment}, instructions::tables::load_tss,
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        //TODO correct way of allocation

        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let start_stack = VirtAddr::from_ptr(unsafe {&STACK});
            let end_stack = start_stack + STACK_SIZE;

            end_stack
        };
        tss
    };

}

lazy_static! {
    static ref GDT: GdtSelectors = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

        GdtSelectors {
            gdt,
            selectors: Selectors {
                code_selector,
                tss_selector,
            },
        }
    };
}

pub fn init() {
    GDT.gdt.load();

    unsafe {
        CS::set_reg(GDT.selectors.code_selector);
        load_tss(GDT.selectors.tss_selector);
    }
}

struct GdtSelectors {
    gdt: GlobalDescriptorTable,
    selectors: Selectors,
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

