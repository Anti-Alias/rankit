import { useState, useEffect, Context, JSX, PropsWithChildren, createContext } from 'react';

// [0, 0.95)        = mobile.
// [0.95, 1.45)     = tablet.
// [1.45, infinity) = desktop.
const ASPECT_RATIO_TABLET: number = 0.95;
const ASPECT_RATIO_DESKTOP: number = 1.45;
type DisplayMode = 'desktop' | 'tablet' | 'mobile';
const DisplayModeContext: Context<DisplayMode> = createContext<DisplayMode>('desktop');


function DisplayModeProvider({children}: PropsWithChildren<{}>): JSX.Element {
    
    const [displayMode, setDisplayMode] = useState<DisplayMode>(computeDisplayMode());

    // Handles resizing
    let currentDisplayMode = displayMode;
    const onResize = () => {
        const nextDisplayMode = computeDisplayMode();
        if(nextDisplayMode != currentDisplayMode) {
            setDisplayMode(nextDisplayMode);
            currentDisplayMode = nextDisplayMode;
        }
    };

    // Registers / deregisters resize listener
    useEffect(() => {
        window.addEventListener('resize', onResize);
        return () =>  window.removeEventListener('resize', onResize);
    }, []);

    // Renders
    return <DisplayModeContext.Provider value={displayMode}>{children}</DisplayModeContext.Provider>;
}

function computeDisplayMode(): DisplayMode {
    const aspectRatio = window.innerWidth / window.innerHeight;
    if(aspectRatio >= ASPECT_RATIO_DESKTOP) {
        return 'desktop';
    }
    else if(aspectRatio >= ASPECT_RATIO_TABLET) {
        return 'tablet';
    }
    else {
        return 'mobile';
    }
}

export { DisplayModeContext, type DisplayMode };
export default DisplayModeProvider;