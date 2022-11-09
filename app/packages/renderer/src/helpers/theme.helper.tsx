class ColourVariable {
  name: string; // --var-name
  public light: string;
  public dark: string;
  styles: string; // required for emotion to allow interpolation

  constructor(name: string, light: string, dark: string) {
    this.name = `--${name}`;
    this.light = light;
    this.dark = dark;
    this.styles = `var(${this.name})`;
  }

  toString() {
    return this.styles;
  }
}

export const colour = (name: string, light: string, dark: string) =>
  new ColourVariable(name, light, dark);

export const darkLightStyles = (
  colours: Record<string, ColourVariable | unknown>
) => {
  const darkStyles: string[] = [];
  const lightStyles: string[] = [];

  const addColours = (colours: Record<string, ColourVariable | unknown>) => {
    for (const value of Object.values(colours)) {
      if (value instanceof ColourVariable) {
        darkStyles.push(`${value.name}: ${value.dark};`);
        lightStyles.push(`${value.name}: ${value.light};`);
      } else {
        addColours(value as Record<string, ColourVariable>);
      }
    }
  };
  addColours(colours);
  return { darkStyles, lightStyles };
};
