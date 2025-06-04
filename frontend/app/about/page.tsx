import { title } from '@/components/primitives'
import { t } from '@/i18n'

export default function AboutPage() {
  return (
    <div>
      <h1 className={title()}>About</h1>
      <p>{t('aboutUnderConstruction')}</p>
    </div>
  )
}
