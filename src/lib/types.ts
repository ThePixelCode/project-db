type project_object = {
    object_id: string;
    object_name: string;
    publication_year: string;
    identifier_id: string;
    language_id: string;
}

export type book = { page_count: number } & project_object
export type music = { duration: string } & project_object
export type videogame = { controller_support: boolean, pegi_id: string } & project_object

export class Optional<T> {
    static STATE = {
        PRESENT: Symbol("present"),
        AUSENT: Symbol("ausent")
    }
    private value: T | undefined
    private state: typeof Optional.STATE.PRESENT | typeof Optional.STATE.AUSENT
    private constructor(t: T | undefined, initial_state: typeof Optional.STATE.PRESENT | typeof Optional.STATE.AUSENT) {
        this.value = t;
        this.state = initial_state
    }

    static empty<T>(): Optional<T> {
        return new Optional<T>(undefined, Optional.STATE.AUSENT)
    }

    static of<T>(t: T): Optional<T> {
        return new Optional<T>(t, Optional.STATE.PRESENT)
    }

    static of_undefinedable<T>(t: T | undefined): Optional<T> {
        if (t === undefined) {
            return Optional.empty()
        }
        return Optional.of(t)
    }

    public is_present(): boolean {
        return this.state === Optional.STATE.PRESENT
    }

    public is_empty(): boolean {
        return this.state === Optional.STATE.AUSENT
    }

    public unwrap(): T {
        if (this.is_empty()) {
            throw new Error("unwrap called on empty")
        }
        return this.value as T
    }
}