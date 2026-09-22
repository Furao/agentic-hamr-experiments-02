//@ HAMR: --platform Microkit --output-dir ../../hamr

package Platform {
    private import HAMR::*;
    private import open_platform::*;

    part def Proc :> Processor {
        // CR-02 generation minimum: 1900 ms component budgets + 180 ms pacer.
        // Deployed schedule and D2 propagation remain subject to W3 validation.
        attribute :>> Frame_Period = 2080 [ms];
        attribute :>> Clock_Period = 2 [ms];
        attribute :>> Max_Domain = 8;
    }

    part def ZCU102_Impl :> System {
        part proc : Proc;
        part seL4 : open_platform::seL4;

        allocation allo :  Deployment_Properties::Actual_Processor_Binding
            allocate seL4 to proc; // (reference (proc)) applies to seL4
    }

    part def ZCU102_Impl_MOCK :> ZCU102_Impl {
        part :>> seL4 : open_platform::seL4_MOCK;
    }

}
