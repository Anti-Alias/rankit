import { JSX, PropsWithChildren, useState } from 'react';


type Theme = {
    'color-text': string,
    
};
type ThemeName = 'light' | 'dark';

const ThemeProvider = ({ children }: PropsWithChildren<ThemeName>): JSX.Element => {

    const [themeName, setThemeName] = useState<ThemeName>('light');
    

    return (<>
        
    </>);
}