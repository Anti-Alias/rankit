import { JSX, PropsWithChildren, useState, createContext, useContext } from 'react';

/** CSS properties of a theme. */
type Theme = {
    colorText:              string;
    colorTextAlt:           string;
    colorOutline:           string;
    colorOutlineFocus:      string;
    colorButton:            string;
    colorButtonSelected:    string;
    colorButtonAlt:         string;
    colorButtonAltSelected: string;
    colorLink:              string;
    colorBackground:        string;
    colorForm:              string;
    colorNavbar:            string;
    filterNavbar:           string;
    filterCard:             string;
    transitionTime:         string;
};

/** Possible theme names */
type ThemeName = 'light' | 'dark';

/** The current theme used by the site */
interface ThemeNameState {
    themeName: ThemeName,
    setThemeName(name: ThemeName): void,
}

const LightTheme: Theme = {
    colorText:                'black',
    colorTextAlt:             'white',
    colorOutline:             'rgb(172, 172, 172)',
    colorOutlineFocus:        'rgb(61, 61, 61)',
    colorButton:              'rgb(178, 87, 206)',
    colorButtonSelected:      'rgb(193, 119, 216)',
    colorButtonAlt:           'rgb(53, 158, 71)',
    colorButtonAltSelected:   'rgb(121, 199, 132)',
    colorLink:                'rgb(0, 162, 255)',
    colorBackground:          'rgb(235, 235, 235)',
    colorForm:                'white',
    colorNavbar:              'white',
    filterNavbar:             'drop-shadow(0 0 3px rgba(0, 0, 0, 0.2))',
    filterCard:               'drop-shadow(2px 2px 1px rgba(0, 0, 0, 0.2))',
    transitionTime:           '0.15s',
};

const DarkTheme: Theme = {
    colorText:                'white',
    colorTextAlt:             'black',
    colorOutline:             'rgb(172, 172, 172)',
    colorOutlineFocus:        'rgb(61, 61, 61)',
    colorButton:              'rgb(178, 87, 206)',
    colorButtonSelected:      'rgb(193, 119, 216)',
    colorButtonAlt:           'rgb(53, 158, 71)',
    colorButtonAltSelected:   'rgb(121, 199, 132)',
    colorLink:                'rgb(0, 162, 255)',
    colorBackground:          'rgb(235, 235, 235)',
    colorForm:                'white',
    colorNavbar:              'white',
    filterNavbar:             'drop-shadow(0 0 3px rgba(0, 0, 0, 0.2))',
    filterCard:               'drop-shadow(2px 2px 1px rgba(0, 0, 0, 0.2))',
    transitionTime:           '0.15s',
};

/** Shareable / settable theme name */
const ThemeNameContext = createContext<ThemeNameState>({
    themeName: 'light',
    setThemeName: () => {}
});

const ThemeProvider = ({ children }: PropsWithChildren<{}>): JSX.Element => {
    
    // State
    const [themeName, setThemeName] = useState<ThemeName>('light');
    const themeState = { themeName, setThemeName };

    // Sets CSS variables using current theme name.
    const theme = themeName === 'light' ? LightTheme : DarkTheme;
    const rootStyle = document.documentElement.style;
    for(const [propName, propValue] of Object.entries(theme)) {
        rootStyle.setProperty(`--${propName}`, propValue);
    }

    // Renders children
    return <ThemeNameContext.Provider value={themeState}>{children}</ThemeNameContext.Provider>;
}

export type { ThemeNameState as ThemeProps };
export default ThemeProvider;