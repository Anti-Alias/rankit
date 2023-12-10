import { JSX, PropsWithChildren, useState, createContext, useContext } from 'react';

/** Possible theme names */
type ThemeName = 'light' | 'dark';

/** State type alias */
type ThemeNameState = [ThemeName, (themeName: ThemeName) => void];

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
    filterButton:           string;
    transitionTime:         string;
};

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
    filterButton:             'drop-shadow(2px 2px 1px rgba(0, 0, 0, 0.2))',
    transitionTime:           '0.15s',
};

const DarkTheme: Theme = {
    colorText:                'white',
    colorTextAlt:             'white',
    colorOutline:             'rgb(172, 172, 172)',
    colorOutlineFocus:        'rgb(61, 61, 61)',
    colorButton:              'rgb(53, 158, 71)',
    colorButtonSelected:      'rgb(121, 199, 132)',
    colorButtonAlt:           'rgb(178, 87, 206)',
    colorButtonAltSelected:   'rgb(193, 119, 216)',
    colorLink:                'rgb(0, 220, 255)',
    colorBackground:          'rgb(70, 70, 70)',
    colorForm:                'rgb(100, 100, 100)',
    colorNavbar:              'rgb(100, 100, 100)',
    filterNavbar:             'drop-shadow(0 0 3px rgba(0, 0, 0, 0.2))',
    filterCard:               'drop-shadow(2px 2px 1px rgba(0, 0, 0, 0.2))',
    filterButton:             'drop-shadow(2px 2px 1px rgba(0, 0, 0, 0.15))',
    transitionTime:           '0.15s',
};

/** Shareable / settable theme name */
const ThemeNameContext = createContext<ThemeNameState>(['light', ()=>{}]);

/** Component that provides a theme name that can be set by child components. */
function ThemeProvider({ children }: PropsWithChildren<{}>): JSX.Element {
    const [themeName, setThemeName] = useState<ThemeName>('light');
    const theme = themeName === 'light' ? LightTheme : DarkTheme;
    for(const [propName, propValue] of Object.entries(theme)) {
        document.documentElement.style.setProperty(`--${propName}`, propValue);
    }
    return <ThemeNameContext.Provider value={[themeName, setThemeName]}>
        {children}
    </ThemeNameContext.Provider>;
}

export { ThemeNameContext, type ThemeNameState };
export default ThemeProvider;