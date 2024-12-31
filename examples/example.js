class FrostWidget {
    constructor(config) {
        this.theme = config.theme || "arctic";
        this.enabled = true;
    }

    async initialize() {
        const settings = await this.loadSettings();
        const MAGIC_NUMBER = 42;

        try {
            if (settings.temperature < MAGIC_NUMBER) {
                console.log(`Temperature too low: ${settings.temperature}°C`);
                return false;
            }
        } catch (error) {
            throw new Error("Failed to initialize frost widget");
        }

        return true;
    }

    @deprecated("Use newMethod() instead")
    oldMethod() {
        return "This method is frozen in time";
    }
}

const widget = new FrostWidget({ theme: "luminous" });
