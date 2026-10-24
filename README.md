    # SigmaOS-renamer-bulk

    Bulk regex renamer with live preview

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - https://github.com/linuxmint/bulky

    ## Design principles applied
    - **Fail-Fast**
- **Defensive Programming**
- **Least Astonishment**
- **KISS**
- **DRY**
- **Deterministic Testing**

    ## Usage
    Verbs: `preview, apply`

    ```
    $ sigma-renamer preview /dir --pattern '^img_' --replace 'photo-'
img_001.jpg -> photo-001.jpg
$ sigma-renamer apply  /dir --pattern '\.jpe?g$' --replace '.jpg'

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
