//! Types for working with STIX vocabularies.
//!
//! From the [specification](https://docs.oasis-open.org/cti/stix/v2.1/cs01/stix-v2.1-cs01.html#_vbsdt43uxrv0):
//!
//! > Some STIX properties are defined using open vocabularies or enumerations.
//! > Enumerations and open vocabularies are defined in STIX in order to enhance interoperability
//! > by increasing the likelihood that different entities use the same exact string to represent
//! > the same concept. If used consistently, open vocabularies make it less likely that one entity
//! > refers to the energy sector as “Energy” and another as “Energy Sector”, thereby making comparison
//! > and correlation easier.

//! An open STIX vocabulary. Vocabularies improver correlation across threat intel from different sources
//! by ensuring exact string equality whenever there is semantic equality.
pub trait Vocabulary {
    /// The vocabulary identifier. This is typically a kebab-case string ending in `-ov`, e.g. `account-type-ov`.
    const TYPE: &'static str;

    /// Whether the contained string is a known value of the vocabulary.
    fn is_known_value(&self) -> bool;
}

stix_derive::vocabulary!(
    #[vocabulary(core)]
    AccountType = [
        facebook,
        ldap,
        nis,
        openid,
        radius,
        skype,
        tacacs,
        twitter,
        unix,
        windows_local,
        windows_domain
    ]
);

stix_derive::vocabulary!(
    #[vocabulary(core)]
    AttackMotivation = [
        accidental,
        coercion,
        dominance,
        ideology,
        notoriety,
        organizational_gain,
        personal_gain,
        personal_satisfaction,
        revenge,
        unpredictable
    ]
);

stix_derive::vocabulary!(
    #[vocabulary(core)]
    AttackResourceLevel = [individual, club, contest, team, organization, government]
);

stix_derive::vocabulary!(
    #[vocabulary(core)]
    IdentityClass = [individual, group, system, organization, class, unknown]
);

stix_derive::vocabulary!(
    #[vocabulary(core)]
    ImplementationLanguage = [
        applescript,
        bash,
        c,
        c_plus_plus = "c++",
        c_sharp = "c#",
        go,
        java,
        javascript,
        lua,
        objective_c,
        perl,
        php,
        powershell,
        python,
        ruby,
        scala,
        swift,
        typescript,
        visual_basic,
        x86_32,
        x86_64
    ]
);

stix_derive::vocabulary!(
    #[vocabulary(core)]
    IndustrySector = [
        agriculture,
        aerospace,
        automotive,
        chemical,
        commercial,
        communications,
        construction,
        defense,
        education,
        energy,
        entertainment,
        financial_services,
        government,
        emergency_services,
        government_local,
        government_national,
        government_public_services,
        government_regional,
        healthcare,
        hospitality_leisure,
        infrastructure,
        dams,
        nuclear,
        water,
        insurance,
        manufacturing,
        mining,
        non_profit,
        pharmaceuticals,
        retail,
        technology,
        telecommunications,
        transportation,
        utilities
    ]
);

stix_derive::vocabulary!(
    #[vocabulary(core)]
    MalwareCapabilities = [
        accesses_remote_machines,
        anti_debugging,
        anti_disassembly,
        anti_emulation,
        anti_memory_forensics,
        anti_sandbox,
        anti_vm,
        captures_input_peripherals,
        captures_output_peripherals,
        captures_system_state_data,
        cleans_traces_of_infection,
        commits_fraud,
        communicates_with_c2,
        compromises_data_availability,
        compromises_data_integrity,
        compromises_system_availability,
        controls_local_machine,
        degrades_security_software,
        degrades_system_updates,
        determines_c2_server,
        emails_spam,
        escalates_privileges,
        evades_av,
        exfiltrates_data,
        fingerprints_host,
        hides_artifacts,
        hides_executing_code,
        infects_files,
        infects_remote_machines,
        installs_other_components,
        persists_after_system_reboot,
        prevents_artifact_access,
        prevents_artifact_deletion,
        probes_network_environment,
        self_modifies,
        steals_authentication_credentials,
        violates_system_operational_integrity
    ]
);

stix_derive::vocabulary!(
    #[vocabulary(core)]
    MalwareType = [
        adware,
        backdoor,
        bot,
        bootkit,
        ddos,
        downloader,
        dropper,
        exploit_kit,
        keylogger,
        ransomware,
        remote_access_trojan,
        resource_exploitation,
        rogue_security_software,
        rootkit,
        screen_capture,
        spyware,
        trojan,
        unknown,
        virus,
        webshell,
        wiper,
        worm
    ]
);

stix_derive::vocabulary!(
    #[vocabulary(core)]
    ProcessorArchitecture = [alpha, arm, ia_64, mips, powerpc, sparc, x86, x86_64]
);
